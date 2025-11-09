use serde_json::Value;

use crate::infrastructure::webserver::{
    connection::{ConnectionId, ConnectionManager},
    context::ApiContext,
    message::{CallBody, WsMessage},
    registry::HandlerRegistry,
};

/// 处理 Call 请求
/// 
/// 根据 method 在 HandlerRegistry 中查找对应的 handler 并执行
pub async fn handle_call(
    conn_id: &ConnectionId,
    call: CallBody,
    registry: &HandlerRegistry,
    conn_mgr: &ConnectionManager,
    ctx: &ApiContext,
) {
    let CallBody { id, method, params } = call;

    let result = if let Some(handler) = registry.get(&method) {
        // 执行注册的 handler
        handler(method.clone(), params.unwrap_or(Value::Null), ctx.clone())
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Unknown method: {}", method))
    };

    let reply = match result {
        Ok(data) => WsMessage::reply_success(id, method, data),
        Err(err) => WsMessage::reply_error(id, method, err),
    };

    if let Err(e) = conn_mgr.send_to(conn_id, reply).await {
        eprintln!("Failed to send reply: {}", e);
    }
}

