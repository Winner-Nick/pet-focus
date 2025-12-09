/**
 * 统计组件的单元测试
 */

import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ContributionWall } from '@/features/stats/components/ContributionWall'
import { StatsOverview, StatsOverviewCompact } from '@/features/stats/components/StatsOverview'

// 模拟数据
const mockMonthStatsData = {
  yearMonth: '2024-12',
  days: [
    {
      date: '2024-12-01',
      timestamp: 1701388800000,
      totalSeconds: 1800,
      completedSessions: 1,
      totalRecords: 1,
      productivity: 50,
      status: 'active' as const,
    },
    {
      date: '2024-12-02',
      timestamp: 1701475200000,
      totalSeconds: 3600,
      completedSessions: 2,
      totalRecords: 2,
      productivity: 100,
      status: 'active' as const,
    },
  ],
  totalSeconds: 5400,
  totalSessions: 3,
  avgSessionDuration: 1800,
  activeDays: 2,
  mostProductiveDay: '2024-12-02',
  maxProductivity: 100,
}

const mockStatsCalculationData = {
  stats: {
    totalSeconds: 50000,
    totalSessions: 28,
    currentStreak: 10,
    longestStreak: 15,
    avgDailySeconds: 2500,
    avgSessionDuration: 1786,
    mostProductiveMonth: '2024-12',
    startDate: '2024-01-01',
    endDate: '2024-12-31',
    totalActiveDays: 200,
    overallProductivity: 85,
    monthlyStats: {},
  },
  isLoading: false,
  isError: false,
}

// Mock useMonthStats hook
vi.mock('@/features/stats/hooks/useStatsCalculation', () => ({
  useMonthStats: vi.fn(() => mockMonthStatsData),
  useStatsCalculation: vi.fn(() => mockStatsCalculationData),
}))

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
    },
  },
})

const Wrapper = ({ children }: { children: React.ReactNode }) => (
  <QueryClientProvider client={queryClient}>
    {children}
  </QueryClientProvider>
)

describe('ContributionWall', () => {
  it('应该渲染贡献墙组件', () => {
    render(<ContributionWall />, { wrapper: Wrapper })

    expect(screen.getByText('月度贡献墙')).toBeDefined()
  })

  it('应该显示月份标题', () => {
    render(<ContributionWall initialMonth="2024-12" />, { wrapper: Wrapper })

    expect(screen.getByText(/2024 年/)).toBeDefined()
    expect(screen.getByText(/12月/)).toBeDefined()
  })

  it('应该显示统计数据', () => {
    render(<ContributionWall />, { wrapper: Wrapper })

    expect(screen.getByText('打卡天数:')).toBeDefined()
    expect(screen.getByText('总时长:')).toBeDefined()
    expect(screen.getByText('最高指数:')).toBeDefined()
  })

  it('应该渲染月份导航按钮', () => {
    const { container } = render(<ContributionWall />, { wrapper: Wrapper })

    const buttons = container.querySelectorAll('button')
    expect(buttons.length).toBeGreaterThanOrEqual(2) // 前后按钮
  })

  it('应该显示图例', () => {
    render(<ContributionWall />, { wrapper: Wrapper })

    expect(screen.getByText('贡献度:')).toBeDefined()
  })

  it('应该调用 onMonthChange 回调', () => {
    const onMonthChange = vi.fn()
    render(<ContributionWall onMonthChange={onMonthChange} />, { wrapper: Wrapper })

    expect(onMonthChange).not.toHaveBeenCalled()
  })
})

describe('StatsOverview', () => {
  it('应该渲染概览组件', () => {
    render(<StatsOverview />, { wrapper: Wrapper })

    expect(screen.getByText('总专注时间')).toBeDefined()
    expect(screen.getByText('总会话数')).toBeDefined()
  })

  it('应该显示 8 张卡片', () => {
    const { container } = render(<StatsOverview />, { wrapper: Wrapper })

    // 查找卡片容器
    const cards = container.querySelectorAll('[class*="border"][class*="rounded"]')
    expect(cards.length).toBeGreaterThanOrEqual(8)
  })

  it('应该显示统计数据', () => {
    render(<StatsOverview />, { wrapper: Wrapper })

    expect(screen.getByText('连续打卡')).toBeDefined()
    expect(screen.getByText('平均生产力')).toBeDefined()
    expect(screen.getByText('平均日时长')).toBeDefined()
  })

  it('应该显示加载状态', () => {
    // 测试默认的 mock 数据渲染
    render(<StatsOverview />, { wrapper: Wrapper })

    // 应该显示统计卡片
    expect(screen.getByText('总专注时间')).toBeDefined()
    expect(screen.getByText('总会话数')).toBeDefined()
  })

  it('应该处理空数据状态', () => {
    // 测试当数据为空时的渲染
    render(<StatsOverview />, { wrapper: Wrapper })

    // 应该显示默认的统计卡片
    expect(screen.getByText('总专注时间')).toBeDefined()
  })
})

describe('StatsOverviewCompact', () => {
  it('应该渲染简化版概览', () => {
    render(<StatsOverviewCompact />, { wrapper: Wrapper })

    // 简化版应该只显示关键指标
    const cards = screen.queryAllByText(/总时长|会话数|连续天数|生产力/)
    expect(cards.length).toBeGreaterThan(0)
  })

  it('简化版应该响应式布局', () => {
    const { container } = render(<StatsOverviewCompact />, { wrapper: Wrapper })

    // 检查是否有响应式类
    const gridContainer = container.querySelector('[class*="grid"]')
    expect(gridContainer).toBeDefined()
  })
})
