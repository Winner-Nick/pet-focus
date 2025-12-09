/**
 * 统计功能模块导出
 */

// Hooks
export { useStatsCalculation, useMonthStats, useDayStats, useWeekStats, useComparisonData, useStreakData, useRefreshStats, useFilteredStats } from "./hooks/useStatsCalculation"

// Components
export { ContributionWall } from "./components/ContributionWall"
export { StatsOverview, StatsOverviewCompact } from "./components/StatsOverview"

// Utilities
export {
  calculateDayStats,
  calculateMonthStats,
  calculateOverallStats,
  buildSessionIndex,
  calculateWeekStats,
  calculateComparison,
  filterStats,
} from "./lib/statsCalculator"

export {
  getCache,
  setCache,
  clearCache,
  updateMonthlyStats,
  updateOverallStats,
  isCacheStale,
  getCacheSize,
  createNewCacheData,
  exportCacheData,
  importCacheData,
  getCacheInfo,
} from "./lib/cacheStorage"

// Types
export type {
  DayStats,
  MonthStats,
  OverallStats,
  SessionIndexEntry,
  StatsCacheData,
  YearStats,
  WeekStats,
  StatsQueryOptions,
  TrendDataPoint,
  ComparisonData,
} from "./types/stats"
