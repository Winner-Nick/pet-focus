export interface Todo {
  id: number
  title: string
  completed: boolean
  created_date: string
  modified_date: string
  due_date: string | null
  remind_before_minutes: number
  notified: boolean
  created_at: string
  updated_at: string
}
