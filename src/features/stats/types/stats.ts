/**
 * 统计功能类型定义
 */

/**
 * 日级统计数据
 */
export interface DayStats {
  /** YYYY-MM-DD 格式的日期字符串 */
  date: string;
  /** 日期时间戳（毫秒） */
  timestamp: number;
  /** 总专注时间（秒） */
  totalSeconds: number;
  /** 完成的专注单位数 */
  completedSessions: number;
  /** 所有记录数 */
  totalRecords: number;
  /** 生产力指数 0-100 */
  productivity: number;
  /** 打卡状态：未打卡 | 打卡 */
  status: "idle" | "active";
}

/**
 * 月级统计数据
 */
export interface MonthStats {
  /** YYYY-MM 格式的年月字符串 */
  yearMonth: string;
  /** 日级统计数据数组 */
  days: DayStats[];
  /** 该月总专注时间（秒） */
  totalSeconds: number;
  /** 该月总会话数 */
  totalSessions: number;
  /** 平均单次专注时长（秒） */
  avgSessionDuration: number;
  /** 打卡天数 */
  activeDays: number;
  /** 最活跃的日期 */
  mostProductiveDay: string | null;
  /** 最高生产力指数 */
  maxProductivity: number;
}

/**
 * 总体统计数据
 */
export interface OverallStats {
  /** 总专注时间（秒） */
  totalSeconds: number;
  /** 总会话数 */
  totalSessions: number;
  /** 当前连续打卡天数 */
  currentStreak: number;
  /** 最长连续打卡天数 */
  longestStreak: number;
  /** 平均日均专注时间（秒） */
  avgDailySeconds: number;
  /** 平均单次专注时长（秒） */
  avgSessionDuration: number;
  /** 最活跃的月份 */
  mostProductiveMonth: string | null;
  /** 第一条记录的日期 */
  startDate: string | null;
  /** 最后一条记录的日期 */
  endDate: string | null;
  /** 有记录的天数 */
  totalActiveDays: number;
  /** 生产力指数（总体） */
  overallProductivity: number;
}

/**
 * Session 索引条目（用于快速查询）
 */
export interface SessionIndexEntry {
  /** Session ID */
  id: number;
  /** 日期戳（毫秒） */
  dateTimestamp: number;
  /** 记录类型 */
  kind: "focus" | "rest";
  /** 记录状态 */
  status: "completed" | "stopped" | "skipped";
  /** 时长（秒） */
  elapsedSeconds: number;
  /** 日期字符串 YYYY-MM-DD */
  dateStr: string;
}

/**
 * 统计缓存数据
 */
export interface StatsCacheData {
  /** 缓存版本 */
  version: number;
  /** 最后更新时间戳 */
  lastUpdateTime: number;
  /** 月度统计数据 Map */
  monthlyStats: Record<string, MonthStats>;
  /** 总体统计数据 */
  overallStats: OverallStats;
  /** Session 索引（用于快速查询） */
  sessionIndex: SessionIndexEntry[];
  /** 缓存有效期（毫秒），默认 24 小时 */
  ttl: number;
}

/**
 * 年度统计视图
 */
export interface YearStats {
  year: number;
  months: MonthStats[];
  totalSeconds: number;
  totalSessions: number;
  totalActiveDays: number;
  avgProductivity: number;
}

/**
 * 周级统计数据
 */
export interface WeekStats {
  /** YYYY-W{weekNumber} 格式 */
  yearWeek: string;
  /** 起始日期 */
  startDate: string;
  /** 结束日期 */
  endDate: string;
  /** 日级统计 */
  days: DayStats[];
  /** 周总时长 */
  totalSeconds: number;
  /** 周总会话数 */
  totalSessions: number;
  /** 打卡天数 */
  activeDays: number;
  /** 平均日时长 */
  avgDailySeconds: number;
}

/**
 * 统计查询条件
 */
export interface StatsQueryOptions {
  /** 开始日期 YYYY-MM-DD */
  startDate?: string;
  /** 结束日期 YYYY-MM-DD */
  endDate?: string;
  /** 指定月份 YYYY-MM */
  yearMonth?: string;
  /** 指定年份 */
  year?: number;
  /** 按种类筛选 */
  kind?: "focus" | "rest" | "all";
  /** 按状态筛选 */
  status?: "completed" | "stopped" | "skipped" | "all";
}

/**
 * 趋势数据点
 */
export interface TrendDataPoint {
  date: string;
  value: number;
  label: string;
}

/**
 * 对比分析数据
 */
export interface ComparisonData {
  /** 当月数据 */
  thisMonth: DayStats[];
  /** 上月数据 */
  lastMonth: DayStats[];
  /** 周对比 */
  weekComparison: {
    thisWeek: WeekStats;
    lastWeek: WeekStats;
  };
  /** 同比增长率 */
  growthRate: number;
}
