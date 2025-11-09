import { useEffect } from "react"
import { listen } from "@tauri-apps/api/event"
import { useQueryClient } from "@tanstack/react-query"

import { todoKeys } from "@/features/todo/api/todo.keys"

type TodoSyncEvent = {
  action?: string
  todoId?: number | null
  source?: "local" | "webserver" | "caldav"
}

export function useTodoSyncEvents() {
  const queryClient = useQueryClient()

  useEffect(() => {
    let disposed = false
    let unsubscribe: (() => void) | null = null

    const setup = async () => {
      try {
        console.log(`[useTodoSyncEvents] 正在注册 todo-data-updated 事件监听器...`)
        
        const off = await listen<TodoSyncEvent>("todo-data-updated", (event) => {
          console.log(`[useTodoSyncEvents] 收到事件:`, event.payload)
          
          if (disposed) {
            console.log(`[useTodoSyncEvents] 已disposed，忽略事件`)
            return
          }
          
          // 只监听外部数据源的变更（WebServer API 或 CalDAV 同步）
          if (event.payload?.source !== "webserver" && event.payload?.source !== "caldav") {
            console.log(`[useTodoSyncEvents] 非外部来源，忽略事件: source=${event.payload?.source}`)
            return
          }

          console.log(`[useTodoSyncEvents] ✅ 触发数据刷新: source=${event.payload?.source}, action=${event.payload?.action}`)
          void queryClient.invalidateQueries({ queryKey: todoKeys.all })

          // 后端已经通过 NotificationManager 发送通知，这里只需刷新数据
          console.log(`[useTodoSyncEvents] 外部数据同步完成`)
        })

        console.log(`[useTodoSyncEvents] ✅ 事件监听器注册成功`)

        if (disposed) {
          off()
        } else {
          unsubscribe = off
        }
      } catch (error) {
        console.error("❌ 监听待办同步事件失败", error)
      }
    }

    void setup()

    return () => {
      disposed = true
      if (unsubscribe) {
        unsubscribe()
      }
    }
  }, [queryClient])
}
