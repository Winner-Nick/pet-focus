/**
 * GitHub 风格的月度贡献墙组件
 */

import { useMemo, useState } from "react"
import { ChevronLeft, ChevronRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip"
import { Badge } from "@/components/ui/badge"
import { useMonthStats } from "../hooks/useStatsCalculation"
import type { DayStats, MonthStats } from "../types/stats"

interface ContributionWallProps {
  initialMonth?: string
  onMonthChange?: (month: string) => void
}

/**
 * 根据秒数获取贡献等级
 * 等级说明：
 * - 0: 0 秒（白色）
 * - 1: 1-30 分钟（浅绿）
 * - 2: 31-60 分钟（中绿）
 * - 3: 61-120 分钟（深绿）
 * - 4: 120+ 分钟（最深绿）
 */
function getContributionLevel(seconds: number): number {
  if (seconds === 0) return 0
  const minutes = Math.floor(seconds / 60)
  if (minutes < 30) return 1
  if (minutes < 60) return 2
  if (minutes < 120) return 3
  return 4
}

/**
 * 获取颜色类名
 */
function getLevelColorClass(level: number): string {
  const colors = [
    "bg-muted", // 0 - 白色/灰色
    "bg-green-200", // 1 - 浅绿
    "bg-green-400", // 2 - 中绿
    "bg-green-600", // 3 - 深绿
    "bg-green-800", // 4 - 最深绿
  ]
  return colors[level] || "bg-muted"
}

/**
 * 格式化秒数为时间字符串
 */
function formatSeconds(seconds: number): string {
  if (seconds === 0) return "0 分钟"
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)

  if (hours > 0) {
    return `${hours} 小时 ${minutes} 分钟`
  }
  return `${minutes} 分钟`
}

/**
 * 获取贡献等级的文字描述
 */
function getLevelDescription(level: number, seconds: number): string {
  const timeStr = formatSeconds(seconds)
  const descriptions = [
    `无贡献 (${timeStr})`,
    `低贡献 - ${timeStr}`,
    `中等贡献 - ${timeStr}`,
    `高贡献 - ${timeStr}`,
    `顶级贡献 - ${timeStr}`,
  ]
  return descriptions[level] || `未知 (${timeStr})`
}

/**
 * 单个日期格子
 */
function ContributionCell({
  dayStats,
}: {
  dayStats: DayStats | null
}) {
  const level = dayStats ? getContributionLevel(dayStats.totalSeconds) : 0
  const colorClass = getLevelColorClass(level)
  const date = dayStats?.date || ""
  const seconds = dayStats?.totalSeconds || 0

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <div
          className={`
            w-3 h-3 rounded-sm cursor-pointer transition-opacity hover:opacity-80
            ${colorClass}
            ${level === 0 ? "border border-muted-foreground/20" : ""}
          `}
        />
      </TooltipTrigger>
      <TooltipContent className="bg-popover text-popover-foreground border border-border">
        <div className="text-xs space-y-1">
          <p className="font-medium">{date}</p>
          <p>{getLevelDescription(level, seconds)}</p>
        </div>
      </TooltipContent>
    </Tooltip>
  )
}

/**
 * 周列容器
 */
function WeekColumn({ days }: { days: (DayStats | null)[] }) {
  return (
    <div className="flex flex-col gap-1">
      {days.map((day, idx) => (
        <ContributionCell key={idx} dayStats={day} />
      ))}
    </div>
  )
}

/**
 * 获取某月的周结构
 */
function getWeekStructure(monthStats: MonthStats | null): (DayStats | null)[][] {
  if (!monthStats || monthStats.days.length === 0) {
    return []
  }

  const days = monthStats.days
  const firstDay = new Date(days[0].date)
  const startDayOfWeek = firstDay.getDay() // 0 = 星期日

  const weeks: (DayStats | null)[][] = []
  let currentWeek: (DayStats | null)[] = []

  // 填充起始空白
  for (let i = 0; i < startDayOfWeek; i++) {
    currentWeek.push(null)
  }

  // 填充天数
  for (const day of days) {
    currentWeek.push(day)
    if (currentWeek.length === 7) {
      weeks.push(currentWeek)
      currentWeek = []
    }
  }

  // 填充结尾空白
  if (currentWeek.length > 0) {
    while (currentWeek.length < 7) {
      currentWeek.push(null)
    }
    weeks.push(currentWeek)
  }

  return weeks
}

/**
 * 月份标题
 */
function MonthHeader({ yearMonth }: { yearMonth: string }) {
  const [year, month] = yearMonth.split("-")
  const monthNames = ["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"]
  return <span className="text-sm font-semibold">{`${year} 年 ${monthNames[parseInt(month) - 1]}`}</span>
}

/**
 * GitHub 贡献墙组件
 */
export function ContributionWall({
  initialMonth = new Date().toISOString().substring(0, 7),
  onMonthChange,
}: ContributionWallProps) {
  const [currentMonth, setCurrentMonth] = useState(initialMonth)
  const monthStats = useMonthStats(currentMonth)

  const weeks = useMemo(() => {
    return getWeekStructure(monthStats)
  }, [monthStats])

  const handlePrevMonth = () => {
    const [year, month] = currentMonth.split("-")
    let y = parseInt(year)
    let m = parseInt(month) - 1

    if (m === 0) {
      m = 12
      y--
    }

    const newMonth = `${y}-${String(m).padStart(2, "0")}`
    setCurrentMonth(newMonth)
    onMonthChange?.(newMonth)
  }

  const handleNextMonth = () => {
    const [year, month] = currentMonth.split("-")
    let y = parseInt(year)
    let m = parseInt(month) + 1

    if (m === 13) {
      m = 1
      y++
    }

    const newMonth = `${y}-${String(m).padStart(2, "0")}`
    setCurrentMonth(newMonth)
    onMonthChange?.(newMonth)
  }

  const activeDays = monthStats?.activeDays ?? 0
  const totalSeconds = monthStats?.totalSeconds ?? 0
  const maxProductivity = monthStats?.maxProductivity ?? 0

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>月度贡献墙</CardTitle>
            <CardDescription>记录你每天的专注时间</CardDescription>
          </div>
          <div className="flex gap-2">
            <Button size="sm" variant="outline" onClick={handlePrevMonth}>
              <ChevronLeft className="size-4" />
            </Button>
            <Button size="sm" variant="outline" onClick={handleNextMonth}>
              <ChevronRight className="size-4" />
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-6">
        {/* 月份标题和统计 */}
        <div className="flex items-center justify-between">
          <MonthHeader yearMonth={currentMonth} />
          <div className="flex gap-4 text-xs text-muted-foreground">
            <div>
              打卡天数: <Badge variant="outline">{activeDays}</Badge>
            </div>
            <div>
              总时长: <Badge variant="outline">{formatSeconds(totalSeconds)}</Badge>
            </div>
            <div>
              最高指数: <Badge variant="outline">{maxProductivity}%</Badge>
            </div>
          </div>
        </div>

        {/* 贡献墙 */}
        <div className="space-y-2">
          {/* 周末标签 */}
          <div className="flex gap-1 text-xs text-muted-foreground mb-2">
            <div className="w-6 text-center">日</div>
            <div className="flex flex-1 gap-1">
              <div className="w-3">一</div>
              <div className="w-3">二</div>
              <div className="w-3">三</div>
              <div className="w-3">四</div>
              <div className="w-3">五</div>
              <div className="w-3">六</div>
              <div className="w-3">日</div>
            </div>
          </div>

          {/* 墙体 */}
          <div className="flex gap-1 overflow-x-auto pb-2">
            {weeks.map((week, weekIdx) => (
              <WeekColumn key={weekIdx} days={week} />
            ))}
          </div>
        </div>

        {/* 图例 */}
        <div className="flex items-center gap-2 text-xs text-muted-foreground pt-4 border-t">
          <span>贡献度:</span>
          <div className="flex gap-1 items-center">
            <div className={`w-3 h-3 rounded-sm ${getLevelColorClass(0)} border border-muted-foreground/20`} />
            <span>无</span>
          </div>
          <div className="flex gap-1 items-center">
            <div className={`w-3 h-3 rounded-sm ${getLevelColorClass(1)}`} />
            <span>低</span>
          </div>
          <div className="flex gap-1 items-center">
            <div className={`w-3 h-3 rounded-sm ${getLevelColorClass(2)}`} />
            <span>中</span>
          </div>
          <div className="flex gap-1 items-center">
            <div className={`w-3 h-3 rounded-sm ${getLevelColorClass(3)}`} />
            <span>高</span>
          </div>
          <div className="flex gap-1 items-center">
            <div className={`w-3 h-3 rounded-sm ${getLevelColorClass(4)}`} />
            <span>顶级</span>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
