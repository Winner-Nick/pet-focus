/**
 * 统计数据概览卡片组件
 */

import { TrendingUp, Flame, Target, Zap } from "lucide-react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { useStatsCalculation } from "../hooks/useStatsCalculation"

/**
 * 格式化秒数为人类可读的时间字符串
 */
function formatSeconds(seconds: number): string {
  if (seconds === 0) return "0 分钟"
  
  const days = Math.floor(seconds / (24 * 3600))
  const hours = Math.floor((seconds % (24 * 3600)) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)

  if (days > 0) {
    return `${days} 天 ${hours} 小时`
  }
  if (hours > 0) {
    return `${hours} 小时 ${minutes} 分钟`
  }
  return `${minutes} 分钟`
}

/**
 * 统计概览卡片
 */
export function StatsOverview() {
  const { stats, isLoading } = useStatsCalculation()

  if (isLoading) {
    return (
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {[1, 2, 3, 4].map((i) => (
          <Card key={i}>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium h-4 bg-muted rounded animate-pulse" />
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold h-8 bg-muted rounded animate-pulse" />
            </CardContent>
          </Card>
        ))}
      </div>
    )
  }

  if (!stats) {
    return (
      <Card className="col-span-full">
        <CardContent className="pt-6">
          <p className="text-center text-muted-foreground">暂无统计数据</p>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      {/* 总专注时间 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium flex items-center gap-2">
            <Zap className="size-4 text-yellow-500" />
            总专注时间
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{formatSeconds(stats.totalSeconds)}</div>
          <p className="text-xs text-muted-foreground mt-1">{stats.totalActiveDays} 个活跃日期</p>
        </CardContent>
      </Card>

      {/* 总会话数 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium flex items-center gap-2">
            <Target className="size-4 text-blue-500" />
            总会话数
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{stats.totalSessions}</div>
          <p className="text-xs text-muted-foreground mt-1">个完成的专注单位</p>
        </CardContent>
      </Card>

      {/* 当前连续天数 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium flex items-center gap-2">
            <Flame className="size-4 text-red-500" />
            连续打卡
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{stats.currentStreak}</div>
          <p className="text-xs text-muted-foreground mt-1">最长: {stats.longestStreak} 天</p>
        </CardContent>
      </Card>

      {/* 平均生产力 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium flex items-center gap-2">
            <TrendingUp className="size-4 text-green-500" />
            平均生产力
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{stats.overallProductivity}%</div>
          <p className="text-xs text-muted-foreground mt-1">基于历史数据计算</p>
        </CardContent>
      </Card>

      {/* 平均日时长 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium">平均日时长</CardTitle>
          <CardDescription>每天平均专注时间</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{formatSeconds(stats.avgDailySeconds)}</div>
        </CardContent>
      </Card>

      {/* 平均单次时长 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium">平均单次时长</CardTitle>
          <CardDescription>每个会话的平均时长</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{formatSeconds(stats.avgSessionDuration)}</div>
        </CardContent>
      </Card>

      {/* 最活跃月份 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium">最活跃月份</CardTitle>
          <CardDescription>专注时间最长的月份</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">
            {stats.mostProductiveMonth ? (
              <Badge>{stats.mostProductiveMonth}</Badge>
            ) : (
              <span className="text-muted-foreground">-</span>
            )}
          </div>
        </CardContent>
      </Card>

      {/* 数据统计 */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-medium">数据统计</CardTitle>
          <CardDescription>记录时间段</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-1 text-xs">
            <div className="flex justify-between">
              <span className="text-muted-foreground">起始:</span>
              <span className="font-medium">{stats.startDate || "-"}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">结束:</span>
              <span className="font-medium">{stats.endDate || "-"}</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

/**
 * 简化版统计卡片（用于不同场景）
 */
export function StatsOverviewCompact() {
  const { stats, isLoading } = useStatsCalculation()

  if (isLoading || !stats) {
    return null
  }

  return (
    <div className="grid gap-2 md:grid-cols-4">
      <div className="p-3 rounded-lg border bg-card">
        <p className="text-xs text-muted-foreground">总时长</p>
        <p className="text-lg font-semibold">{formatSeconds(stats.totalSeconds)}</p>
      </div>
      <div className="p-3 rounded-lg border bg-card">
        <p className="text-xs text-muted-foreground">会话数</p>
        <p className="text-lg font-semibold">{stats.totalSessions}</p>
      </div>
      <div className="p-3 rounded-lg border bg-card">
        <p className="text-xs text-muted-foreground">连续天数</p>
        <p className="text-lg font-semibold">{stats.currentStreak}</p>
      </div>
      <div className="p-3 rounded-lg border bg-card">
        <p className="text-xs text-muted-foreground">生产力</p>
        <p className="text-lg font-semibold">{stats.overallProductivity}%</p>
      </div>
    </div>
  )
}
