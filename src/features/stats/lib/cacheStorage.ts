/**
 * 缓存存储和管理
 */

import type { StatsCacheData, MonthStats, OverallStats } from "../types/stats"

const CACHE_KEY = "pet-focus:stats:cache"
const CACHE_TTL = 24 * 60 * 60 * 1000 // 24 小时
const CURRENT_CACHE_VERSION = 1

/**
 * 初始化缓存数据
 */
function initializeCacheData(): StatsCacheData {
  return {
    version: CURRENT_CACHE_VERSION,
    lastUpdateTime: Date.now(),
    monthlyStats: {},
    overallStats: {
      totalSeconds: 0,
      totalSessions: 0,
      currentStreak: 0,
      longestStreak: 0,
      avgDailySeconds: 0,
      avgSessionDuration: 0,
      mostProductiveMonth: null,
      startDate: null,
      endDate: null,
      totalActiveDays: 0,
      overallProductivity: 0,
    },
    sessionIndex: [],
    ttl: CACHE_TTL,
  }
}

/**
 * 从 LocalStorage 获取缓存
 */
export function getCache(): StatsCacheData | null {
  try {
    const cached = localStorage.getItem(CACHE_KEY)
    if (!cached) return null

    const data = JSON.parse(cached) as StatsCacheData

    // 检查版本
    if (data.version !== CURRENT_CACHE_VERSION) {
      console.warn("Cache version mismatch, clearing cache")
      clearCache()
      return null
    }

    // 检查过期
    if (Date.now() - data.lastUpdateTime > data.ttl) {
      console.warn("Cache expired")
      clearCache()
      return null
    }

    return data
  } catch (error) {
    console.error("Failed to parse cache:", error)
    clearCache()
    return null
  }
}

/**
 * 保存缓存到 LocalStorage
 */
export function setCache(data: StatsCacheData): void {
  try {
    data.lastUpdateTime = Date.now()
    localStorage.setItem(CACHE_KEY, JSON.stringify(data))
  } catch (error) {
    console.error("Failed to save cache:", error)
    // 如果是存储空间满的错误，尝试清理一些数据
    if (error instanceof DOMException && error.code === 22) {
      console.warn("LocalStorage quota exceeded, clearing cache")
      clearCache()
    }
  }
}

/**
 * 清除缓存
 */
export function clearCache(): void {
  try {
    localStorage.removeItem(CACHE_KEY)
  } catch (error) {
    console.error("Failed to clear cache:", error)
  }
}

/**
 * 更新缓存中的月度统计
 */
export function updateMonthlyStats(
  cacheData: StatsCacheData,
  yearMonth: string,
  stats: MonthStats
): void {
  cacheData.monthlyStats[yearMonth] = stats
}

/**
 * 更新缓存中的总体统计
 */
export function updateOverallStats(
  cacheData: StatsCacheData,
  stats: OverallStats
): void {
  cacheData.overallStats = stats
}

/**
 * 检查缓存是否需要更新
 */
export function isCacheStale(lastSessionUpdateTime: number): boolean {
  const cache = getCache()
  if (!cache) return true

  // 如果最后一条 Session 的更新时间晚于缓存时间，说明缓存过期
  return lastSessionUpdateTime > cache.lastUpdateTime
}

/**
 * 获取缓存大小（字节）
 */
export function getCacheSize(): number {
  try {
    const cached = localStorage.getItem(CACHE_KEY)
    if (!cached) return 0
    return new Blob([cached]).size
  } catch (error) {
    console.error("Failed to get cache size:", error)
    return 0
  }
}

/**
 * 创建新的缓存数据结构
 */
export function createNewCacheData(): StatsCacheData {
  return initializeCacheData()
}

/**
 * 批量导出缓存数据
 */
export function exportCacheData(): string {
  const cache = getCache()
  if (!cache) {
    throw new Error("No cache data to export")
  }
  return JSON.stringify(cache, null, 2)
}

/**
 * 导入缓存数据
 */
export function importCacheData(jsonData: string): void {
  try {
    const data = JSON.parse(jsonData) as StatsCacheData

    // 验证数据结构
    if (!data.version || !data.monthlyStats || !data.overallStats) {
      throw new Error("Invalid cache data structure")
    }

    setCache(data)
  } catch (error) {
    console.error("Failed to import cache data:", error)
    throw error
  }
}

/**
 * 获取缓存统计信息
 */
export function getCacheInfo(): {
  exists: boolean
  size: number
  lastUpdate: Date | null
  ttl: number
  isStale: boolean
} {
  const cache = getCache()
  const cacheSize = getCacheSize()
  const isStale = cache ? Date.now() - cache.lastUpdateTime > CACHE_TTL : true

  return {
    exists: cache !== null,
    size: cacheSize,
    lastUpdate: cache ? new Date(cache.lastUpdateTime) : null,
    ttl: CACHE_TTL,
    isStale,
  }
}
