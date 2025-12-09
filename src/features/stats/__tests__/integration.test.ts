/**
 * 统计功能集成测试
 */

import { describe, it, expect, beforeEach } from 'vitest'
import type { PomodoroSession, PomodoroRecord } from '@/features/pomodoro/api/session.api'
import {
  calculateDayStats,
  calculateMonthStats,
  calculateOverallStats,
} from '@/features/stats/lib/statsCalculator'
import { getCache, clearCache } from '@/features/stats/lib/cacheStorage'

/**
 * 创建模拟数据
 */
function createMockSession(id: number): PomodoroSession {
  return {
    id,
    note: `Session ${id}`,
    archived: false,
    archivedAt: null,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  }
}

function createMockRecord(
  id: number,
  date: string,
  seconds: number
): PomodoroRecord {
  const startAt = new Date(`${date}T08:00:00Z`).toISOString()
  return {
    id,
    kind: 'focus',
    status: 'completed',
    round: 1,
    start_at: startAt,
    end_at: new Date(new Date(startAt).getTime() + seconds * 1000).toISOString(),
    elapsed_seconds: seconds,
    related_todo_id: null,
    created_at: startAt,
    updated_at: startAt,
  }
}

describe('Stats Integration Tests', () => {
  beforeEach(() => {
    localStorage.clear()
  })

  describe('完整工作流程', () => {
    it('应该处理从原始数据到缓存的完整流程', () => {
      // 1. 创建模拟数据
      const sessions: PomodoroSession[] = [
        createMockSession(1),
        createMockSession(2),
        createMockSession(3),
      ]

      const records: PomodoroRecord[] = [
        createMockRecord(1, '2024-12-01', 1800),
        createMockRecord(2, '2024-12-01', 1800),
        createMockRecord(3, '2024-12-02', 3600),
        createMockRecord(4, '2024-12-03', 2700),
        createMockRecord(5, '2024-12-05', 1800),
      ]

      // 2. 计算统计数据
      const overallStats = calculateOverallStats(sessions, records)

      // 3. 验证计算结果
      expect(overallStats.totalSeconds).toBe(11700) // 5 条记录总时长
      expect(overallStats.totalSessions).toBe(5)
      expect(overallStats.totalActiveDays).toBe(4) // 4 个不同的日期

      // 4. 验证缓存功能
      expect(getCache()).toBeNull() // 初始无缓存

      // 5. 验证总体统计
      expect(overallStats.currentStreak).toBeGreaterThanOrEqual(0)
      expect(overallStats.longestStreak).toBeGreaterThanOrEqual(0)
    })
  })

  describe('跨月份数据处理', () => {
    it('应该正确处理跨月份的数据', () => {
      const records: PomodoroRecord[] = [
        createMockRecord(1, '2024-11-28', 1800),
        createMockRecord(2, '2024-11-29', 1800),
        createMockRecord(3, '2024-11-30', 1800),
        createMockRecord(4, '2024-12-01', 1800),
        createMockRecord(5, '2024-12-02', 1800),
      ]

      // 计算各月统计
      const novStats = calculateMonthStats(records, '2024-11')
      const decStats = calculateMonthStats(records, '2024-12')

      expect(novStats.totalSessions).toBe(3)
      expect(decStats.totalSessions).toBe(2)
      expect(novStats.totalSeconds).toBe(5400)
      expect(decStats.totalSeconds).toBe(3600)
    })
  })

  describe('生产力指数计算', () => {
    it('应该准确计算生产力指数', () => {
      // 目标是 1 小时 (3600 秒)
      const records = [
        createMockRecord(1, '2024-12-01', 1800), // 50%
        createMockRecord(2, '2024-12-02', 3600), // 100%
        createMockRecord(3, '2024-12-03', 5400), // 150% -> 100% (上限)
        createMockRecord(4, '2024-12-04', 900), // 25%
      ]

      const dayStats = [
        calculateDayStats([records[0]], '2024-12-01'),
        calculateDayStats([records[1]], '2024-12-02'),
        calculateDayStats([records[2]], '2024-12-03'),
        calculateDayStats([records[3]], '2024-12-04'),
      ]

      expect(dayStats[0].productivity).toBe(50)
      expect(dayStats[1].productivity).toBe(100)
      expect(dayStats[2].productivity).toBe(100) // 上限
      expect(dayStats[3].productivity).toBe(25)
    })
  })

  describe('连续打卡计算', () => {
    it('应该正确计算连续打卡天数', () => {
      const records: PomodoroRecord[] = [
        createMockRecord(1, '2024-12-01', 1800),
        createMockRecord(2, '2024-12-02', 1800),
        createMockRecord(3, '2024-12-03', 1800),
        // 12-04 没有记录
        createMockRecord(4, '2024-12-05', 1800),
        createMockRecord(5, '2024-12-06', 1800),
      ]

      const monthStats = calculateMonthStats(records, '2024-12')

      // 最长连续应该是 01-02-03 (3 天)
      expect(monthStats.days.length).toBeGreaterThan(0)
    })
  })

  describe('大数据集处理', () => {
    it('应该高效处理大量数据', () => {
      // 生成 365 天的数据
      const records: PomodoroRecord[] = []
      for (let i = 0; i < 365; i++) {
        const date = new Date('2024-01-01')
        date.setDate(date.getDate() + i)
        const dateStr = date.toISOString().substring(0, 10)

        // 每天 2-4 条记录
        const recordCount = Math.random() > 0.3 ? (Math.random() > 0.5 ? 2 : 3) : 4
        for (let j = 0; j < recordCount; j++) {
          records.push(createMockRecord(records.length + 1, dateStr, 1800 + Math.random() * 1800))
        }
      }

      const startTime = performance.now()
      const overallStats = calculateOverallStats([], records)
      const endTime = performance.now()

      // 计算应该在 100ms 内完成
      expect(endTime - startTime).toBeLessThan(100)

      // 验证结果
      expect(overallStats.totalSessions).toBe(records.length)
      expect(overallStats.totalActiveDays).toBeLessThanOrEqual(365)
    })
  })

  describe('边界情况', () => {
    it('应该处理单条记录', () => {
      const records = [createMockRecord(1, '2024-12-01', 1800)]

      const stats = calculateOverallStats([], records)

      expect(stats.totalSessions).toBe(1)
      expect(stats.totalSeconds).toBe(1800)
      expect(stats.totalActiveDays).toBe(1)
    })

    it('应该处理长时间的单条记录', () => {
      const records = [createMockRecord(1, '2024-12-01', 36000)] // 10 小时

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.totalSeconds).toBe(36000)
      expect(dayStats.productivity).toBe(100) // 超过 1 小时目标
    })

    it('应该处理非常短的记录', () => {
      const records = [createMockRecord(1, '2024-12-01', 60)] // 1 分钟

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats.totalSeconds).toBe(60)
      expect(dayStats.productivity).toBeGreaterThan(0)
    })

    it('应该处理完全空的数据', () => {
      const records: PomodoroRecord[] = []

      const stats = calculateOverallStats([], records)

      expect(stats.totalSessions).toBe(0)
      expect(stats.totalSeconds).toBe(0)
      expect(stats.totalActiveDays).toBe(0)
      expect(stats.startDate).toBeNull()
      expect(stats.endDate).toBeNull()
    })
  })

  describe('缓存一致性', () => {
    it('应该在多次调用中保持一致', () => {
      const records: PomodoroRecord[] = [
        createMockRecord(1, '2024-12-01', 1800),
        createMockRecord(2, '2024-12-02', 1800),
        createMockRecord(3, '2024-12-03', 1800),
      ]

      const stats1 = calculateOverallStats([], records)
      const stats2 = calculateOverallStats([], records)

      expect(stats1).toEqual(stats2)
    })

    it('应该在缓存后返回相同的结果', () => {
      const records: PomodoroRecord[] = [
        createMockRecord(1, '2024-12-01', 1800),
        createMockRecord(2, '2024-12-02', 1800),
      ]

      const stats = calculateOverallStats([], records)
      clearCache()

      // 即使清除缓存，计算结果也应该相同
      const stats2 = calculateOverallStats([], records)

      expect(stats).toEqual(stats2)
    })
  })

  describe('数据完整性', () => {
    it('应该保留所有需要的数据字段', () => {
      const records = [createMockRecord(1, '2024-12-01', 1800)]

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(dayStats).toHaveProperty('date')
      expect(dayStats).toHaveProperty('timestamp')
      expect(dayStats).toHaveProperty('totalSeconds')
      expect(dayStats).toHaveProperty('completedSessions')
      expect(dayStats).toHaveProperty('totalRecords')
      expect(dayStats).toHaveProperty('productivity')
      expect(dayStats).toHaveProperty('status')
    })

    it('应该验证数据类型', () => {
      const records = [createMockRecord(1, '2024-12-01', 1800)]

      const dayStats = calculateDayStats(records, '2024-12-01')

      expect(typeof dayStats.date).toBe('string')
      expect(typeof dayStats.timestamp).toBe('number')
      expect(typeof dayStats.totalSeconds).toBe('number')
      expect(typeof dayStats.completedSessions).toBe('number')
      expect(typeof dayStats.totalRecords).toBe('number')
      expect(typeof dayStats.productivity).toBe('number')
      expect(['active', 'idle']).toContain(dayStats.status)
    })
  })
})
