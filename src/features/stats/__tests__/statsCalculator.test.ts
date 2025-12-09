/**
 * 统计计算函数的单元测试
 */

import { describe, it, expect } from 'vitest'
import type { PomodoroRecord } from '@/features/pomodoro/api/session.api'
import {
  calculateDayStats,
  calculateMonthStats,
  calculateOverallStats,
  calculateCurrentStreak,
  calculateLongestStreak,
  calculateWeekStats,
  filterStats,
  buildSessionIndex,
} from '@/features/stats/lib/statsCalculator'

/**
 * 创建测试数据
 */
function createMockRecord(
  id: number,
  startAt: string,
  elapsedSeconds: number,
  kind: 'focus' | 'rest' = 'focus',
  status: 'completed' | 'stopped' | 'skipped' = 'completed'
): PomodoroRecord {
  return {
    id,
    kind,
    status,
    round: 1,
    start_at: startAt,
    end_at: new Date(new Date(startAt).getTime() + elapsedSeconds * 1000).toISOString(),
    elapsed_seconds: elapsedSeconds,
    related_todo_id: null,
    created_at: startAt,
    updated_at: startAt,
  }
}

describe('statsCalculator', () => {
  describe('calculateDayStats', () => {
    it('应该计算单日的统计数据', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800), // 30 分钟
        createMockRecord(2, '2024-12-01T09:00:00Z', 1500), // 25 分钟
        createMockRecord(3, '2024-12-01T10:00:00Z', 300, 'rest'), // 休息
      ]

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.date).toBe('2024-12-01')
      expect(dayStats.totalSeconds).toBe(3300) // 30 + 25 分钟 (只计算 focus)
      expect(dayStats.completedSessions).toBe(2)
      expect(dayStats.totalRecords).toBe(3)
      expect(dayStats.status).toBe('active')
      expect(dayStats.productivity).toBeGreaterThan(0)
    })

    it('应该正确处理无数据的日期', () => {
      const records: PomodoroRecord[] = []
      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.date).toBe('2024-12-01')
      expect(dayStats.totalSeconds).toBe(0)
      expect(dayStats.completedSessions).toBe(0)
      expect(dayStats.status).toBe('idle')
      expect(dayStats.productivity).toBe(0)
    })

    it('应该只计算 focus 类型的记录', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800, 'focus'),
        createMockRecord(2, '2024-12-01T09:00:00Z', 600, 'rest'),
        createMockRecord(3, '2024-12-01T10:00:00Z', 1200, 'rest'),
      ]

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.totalSeconds).toBe(1800) // 只计算 focus
      expect(dayStats.completedSessions).toBe(1)
    })

    it('应该只统计当天的记录', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-02T08:00:00Z', 1800), // 不同日期
      ]

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.totalSeconds).toBe(1800)
      expect(dayStats.completedSessions).toBe(1)
    })

    it('应该根据时长计算生产力指数', () => {
      // 30 分钟 = 1800 秒，目标 1 小时 = 3600 秒，应该是 50%
      const records = [createMockRecord(1, '2024-12-01T08:00:00Z', 1800)]
      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.productivity).toBe(50)
    })

    it('生产力指数不应该超过 100', () => {
      // 2 小时 = 7200 秒，超过 1 小时目标
      const records = [createMockRecord(1, '2024-12-01T08:00:00Z', 7200)]
      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.productivity).toBe(100)
    })
  })

  describe('calculateMonthStats', () => {
    it('应该计算月度的统计数据', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-02T08:00:00Z', 1800),
        createMockRecord(3, '2024-12-03T08:00:00Z', 1800),
        createMockRecord(4, '2024-12-05T08:00:00Z', 1800), // 跳过 04
      ]

      const monthStats = calculateMonthStats(records, '2024-12')

      expect(monthStats.yearMonth).toBe('2024-12')
      expect(monthStats.totalSeconds).toBe(7200) // 4 * 1800
      expect(monthStats.totalSessions).toBe(4)
      expect(monthStats.activeDays).toBe(4)
      expect(monthStats.days.length).toBe(4)
    })

    it('应该找到最活跃的日期', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800), // 50%
        createMockRecord(2, '2024-12-02T08:00:00Z', 3600), // 100%
        createMockRecord(3, '2024-12-03T08:00:00Z', 900), // 25%
      ]

      const monthStats = calculateMonthStats(records, '2024-12')

      expect(monthStats.mostProductiveDay).toBe('2024-12-02')
      expect(monthStats.maxProductivity).toBe(100)
    })

    it('应该计算平均单次时长', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-01T09:00:00Z', 1200),
      ]

      const monthStats = calculateMonthStats(records, '2024-12')

      // 总时长 3000 秒，2 个会话，平均 1500 秒
      expect(monthStats.avgSessionDuration).toBe(1500)
    })

    it('应该正确处理空月份', () => {
      const records: PomodoroRecord[] = []
      const monthStats = calculateMonthStats(records, '2024-12')

      expect(monthStats.yearMonth).toBe('2024-12')
      expect(monthStats.days.length).toBe(0)
      expect(monthStats.totalSeconds).toBe(0)
      expect(monthStats.activeDays).toBe(0)
    })
  })

  describe('calculateOverallStats', () => {
    it('应该计算总体统计数据', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-02T08:00:00Z', 1800),
        createMockRecord(3, '2024-11-30T08:00:00Z', 1800),
      ]

      const overallStats = calculateOverallStats([], records)

      expect(overallStats.totalSeconds).toBe(5400)
      expect(overallStats.totalSessions).toBe(3)
      expect(overallStats.totalActiveDays).toBe(3)
      expect(overallStats.startDate).toBe('2024-11-30')
      expect(overallStats.endDate).toBe('2024-12-02')
    })

    it('应该计算平均日时长', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 3600), // 1 小时
        createMockRecord(2, '2024-12-02T08:00:00Z', 3600), // 1 小时
      ]

      const overallStats = calculateOverallStats([], records)

      // 总 7200 秒，2 天活跃，平均 3600 秒/天
      expect(overallStats.avgDailySeconds).toBe(3600)
    })

    it('应该计算最活跃月份', () => {
      const records = [
        createMockRecord(1, '2024-11-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-01T08:00:00Z', 3600),
        createMockRecord(3, '2024-12-02T08:00:00Z', 3600),
      ]

      const overallStats = calculateOverallStats([], records)

      expect(overallStats.mostProductiveMonth).toBe('2024-12')
    })
  })

  describe('calculateCurrentStreak', () => {
    it('应该计算连续打卡天数', () => {
      const baseDate = new Date('2024-12-05')
      const records = [
        createMockRecord(1, '2024-12-03T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-04T08:00:00Z', 1800),
        createMockRecord(3, '2024-12-05T08:00:00Z', 1800),
      ]

      const dayStats = records.map((r) => {
        const dateStr = r.start_at.substring(0, 10)
        return {
          date: dateStr,
          timestamp: new Date(dateStr).getTime(),
          totalSeconds: r.elapsed_seconds,
          completedSessions: 1,
          totalRecords: 1,
          productivity: 50,
          status: 'active' as const,
        }
      })

      const streak = calculateCurrentStreak(dayStats, baseDate)

      expect(streak).toBe(3)
    })

    it('应该在遇到非活跃日期时停止计算', () => {
      const baseDate = new Date('2024-12-05')
      const dayStats = [
        { date: '2024-12-03', timestamp: new Date('2024-12-03').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-04', timestamp: new Date('2024-12-04').getTime(), totalSeconds: 0, completedSessions: 0, totalRecords: 0, productivity: 0, status: 'idle' as const },
        { date: '2024-12-05', timestamp: new Date('2024-12-05').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
      ]

      const streak = calculateCurrentStreak(dayStats, baseDate)

      expect(streak).toBe(1) // 只有 12-05
    })

    it('应该在没有数据时返回 0', () => {
      const dayStats = []
      const streak = calculateCurrentStreak(dayStats, new Date('2024-12-05'))

      expect(streak).toBe(0)
    })
  })

  describe('calculateLongestStreak', () => {
    it('应该找到最长连续打卡天数', () => {
      const dayStats = [
        { date: '2024-12-01', timestamp: new Date('2024-12-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-02', timestamp: new Date('2024-12-02').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-03', timestamp: new Date('2024-12-03').getTime(), totalSeconds: 0, completedSessions: 0, totalRecords: 0, productivity: 0, status: 'idle' as const },
        { date: '2024-12-04', timestamp: new Date('2024-12-04').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-05', timestamp: new Date('2024-12-05').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-06', timestamp: new Date('2024-12-06').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
      ]

      const longestStreak = calculateLongestStreak(dayStats)

      expect(longestStreak).toBe(3) // 04, 05, 06 三天
    })

    it('应该处理全部非活跃的情况', () => {
      const dayStats = [
        { date: '2024-12-01', timestamp: new Date('2024-12-01').getTime(), totalSeconds: 0, completedSessions: 0, totalRecords: 0, productivity: 0, status: 'idle' as const },
        { date: '2024-12-02', timestamp: new Date('2024-12-02').getTime(), totalSeconds: 0, completedSessions: 0, totalRecords: 0, productivity: 0, status: 'idle' as const },
      ]

      const longestStreak = calculateLongestStreak(dayStats)

      expect(longestStreak).toBe(0)
    })
  })

  describe('calculateWeekStats', () => {
    it('应该计算周级统计数据', () => {
      const records = [
        createMockRecord(1, '2024-12-02T08:00:00Z', 1800), // 星期一
        createMockRecord(2, '2024-12-03T08:00:00Z', 1800), // 星期二
        createMockRecord(3, '2024-12-04T08:00:00Z', 1800), // 星期三
      ]

      const weekStats = calculateWeekStats(records, '2024-12-02')

      expect(weekStats.totalSeconds).toBe(5400)
      expect(weekStats.totalSessions).toBe(3)
      expect(weekStats.activeDays).toBe(3)
      expect(weekStats.days.length).toBe(7) // 一周有 7 天
    })
  })

  describe('buildSessionIndex', () => {
    it('应该构建会话索引', () => {
      const records = [
        createMockRecord(1, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-02T08:00:00Z', 1800),
      ]

      const index = buildSessionIndex(records)

      expect(index.length).toBe(2)
      expect(index[0].id).toBe(1)
      expect(index[0].dateStr).toBe('2024-12-01')
      expect(index[0].elapsedSeconds).toBe(1800)
    })

    it('应该按日期排序索引', () => {
      const records = [
        createMockRecord(1, '2024-12-03T08:00:00Z', 1800),
        createMockRecord(2, '2024-12-01T08:00:00Z', 1800),
        createMockRecord(3, '2024-12-02T08:00:00Z', 1800),
      ]

      const index = buildSessionIndex(records)

      expect(index[0].dateStr).toBe('2024-12-01')
      expect(index[1].dateStr).toBe('2024-12-02')
      expect(index[2].dateStr).toBe('2024-12-03')
    })
  })

  describe('filterStats', () => {
    it('应该按日期范围筛选', () => {
      const stats = [
        { date: '2024-12-01', timestamp: new Date('2024-12-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-02', timestamp: new Date('2024-12-02').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-03', timestamp: new Date('2024-12-03').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
      ]

      const filtered = filterStats(stats, {
        startDate: '2024-12-02',
        endDate: '2024-12-03',
      })

      expect(filtered.length).toBe(2)
      expect(filtered[0].date).toBe('2024-12-02')
    })

    it('应该按年月筛选', () => {
      const stats = [
        { date: '2024-11-30', timestamp: new Date('2024-11-30').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-01', timestamp: new Date('2024-12-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-02', timestamp: new Date('2024-12-02').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
      ]

      const filtered = filterStats(stats, { yearMonth: '2024-12' })

      expect(filtered.length).toBe(2)
    })

    it('应该按年份筛选', () => {
      const stats = [
        { date: '2023-12-01', timestamp: new Date('2023-12-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-01-01', timestamp: new Date('2024-01-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
        { date: '2024-12-01', timestamp: new Date('2024-12-01').getTime(), totalSeconds: 1800, completedSessions: 1, totalRecords: 1, productivity: 50, status: 'active' as const },
      ]

      const filtered = filterStats(stats, { year: 2024 })

      expect(filtered.length).toBe(2)
    })
  })
})
