use std::sync::{atomic::{AtomicU64, Ordering}, Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::infrastructure::webserver::{
    context::ApiContext,
    handler::handle_call,
    message::WsMessage,
    registry::HandlerRegistry,
};

static CONN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Router 状态（包含 Context 和 HandlerRegistry）
#[derive(Clone)]
struct RouterState {
    ctx: ApiContext,
    registry: Arc<HandlerRegistry>,
}

pub(super) fn build_router(ctx: ApiContext, registry: HandlerRegistry) -> Router {
    let state = RouterState {
        ctx,
        registry: Arc::new(registry),
    };

    Router::new()
        .route("/ws", any(ws_handler))
        .with_state(state)
}

/// WebSocket 升级处理器
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<RouterState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// 处理 WebSocket 连接
async fn handle_socket(socket: WebSocket, state: RouterState) {
    let (mut sender, mut receiver) = socket.split();

    let RouterState { ctx, registry } = state;
    let conn_mgr = ctx.connection_manager();

    // 生成唯一连接 ID
    let conn_id = format!("conn-{}", CONN_COUNTER.fetch_add(1, Ordering::SeqCst));

    // 创建接收通道
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // 注册连接
    conn_mgr.register(conn_id.clone(), tx).await;
    println!("WebSocket connection registered: {}", conn_id);

    // 发送任务：从通道接收消息并发送到 WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // 接收任务
    let conn_id_clone = conn_id.clone();
    let conn_mgr_clone = conn_mgr.clone();
    let ctx_clone = ctx.clone();
    let registry_clone = registry.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(WsMessage::Call { body: call }) => {
                        handle_call(&conn_id_clone, call, &registry_clone, &conn_mgr_clone, &ctx_clone).await;
                    }
                    Ok(WsMessage::Listen { body: listen }) => {
                        let channel = listen.channel;
                        
                        // 验证事件是否已注册
                        if registry_clone.is_event_registered(&channel) {
                            conn_mgr_clone
                                .subscribe(&conn_id_clone, channel.clone())
                                .await;
                            println!("Connection {} subscribed to {}", conn_id_clone, channel);
                            
                            // 发送订阅成功响应
                            let response = WsMessage::reply_success(
                                format!("listen-{}", channel),
                                "listen".to_string(),
                                serde_json::json!({
                                    "subscribed": true,
                                    "event": channel
                                })
                            );
                            let _ = conn_mgr_clone.send_to(&conn_id_clone, response).await;
                        } else {
                            // 发送订阅失败响应
                            eprintln!("Connection {} attempted to subscribe to unregistered event: {}", conn_id_clone, channel);
                            let response = WsMessage::reply_error(
                                format!("listen-{}", channel),
                                "listen".to_string(),
                                format!("Event '{}' is not registered", channel)
                            );
                            let _ = conn_mgr_clone.send_to(&conn_id_clone, response).await;
                        }
                    }
                    Ok(_) => {
                        eprintln!("Unexpected message type from client");
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                    }
                }
            } else if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // 等待任意任务完成
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // 清理连接
    conn_mgr.unregister(&conn_id).await;
    println!("WebSocket connection closed: {}", conn_id);
}
