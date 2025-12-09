#!/usr/bin/env pwsh

# 统计功能测试安装和运行脚本
# Windows PowerShell 版本

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "统计功能测试框架安装和运行" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# 颜色定义
$SUCCESS = "Green"
$ERROR_COLOR = "Red"
$INFO = "Cyan"
$WARNING = "Yellow"

function Write-Info {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $INFO
}

function Write-Success {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $SUCCESS
}

function Write-Error-Message {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $ERROR_COLOR
}

function Write-Warning-Message {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $WARNING
}

# 检查 pnpm 是否已安装
Write-Info "`n[1/5] 检查 pnpm 是否已安装..."
$pnpmPath = Get-Command pnpm -ErrorAction SilentlyContinue
if (-not $pnpmPath) {
    Write-Error-Message "❌ 未找到 pnpm。请先安装 pnpm:"
    Write-Host "  npm install -g pnpm" -ForegroundColor Yellow
    exit 1
}
Write-Success "✅ pnpm 已安装: $($pnpmPath.Source)"

# 检查 Node 版本
Write-Info "`n[2/5] 检查 Node 版本..."
$nodeVersion = & node --version
Write-Success "✅ Node 版本: $nodeVersion"

# 安装测试依赖
Write-Info "`n[3/5] 安装测试依赖..."
Write-Info "运行: pnpm install -D vitest @testing-library/react @testing-library/user-event jsdom @vitest/ui"
& pnpm install -D vitest '@testing-library/react' '@testing-library/user-event' jsdom '@vitest/ui'

if ($LASTEXITCODE -ne 0) {
    Write-Error-Message "❌ 依赖安装失败"
    exit 1
}
Write-Success "✅ 依赖安装成功"

# 验证安装
Write-Info "`n[4/5] 验证测试依赖安装..."
$dependencies = @('vitest', '@testing-library/react', '@testing-library/user-event', 'jsdom')
$allInstalled = $true

foreach ($dep in $dependencies) {
    $installed = & pnpm list $dep --depth=0 2>&1 | Select-String -Pattern $dep -Quiet
    if ($installed) {
        Write-Success "✅ $dep"
    } else {
        Write-Error-Message "❌ $dep 未安装"
        $allInstalled = $false
    }
}

if (-not $allInstalled) {
    Write-Error-Message "`n❌ 某些依赖未正确安装"
    Write-Warning-Message "请手动运行:"
    Write-Host "  pnpm install -D vitest @testing-library/react @testing-library/user-event jsdom @vitest/ui" -ForegroundColor Yellow
    exit 1
}

# 提示后续步骤
Write-Info "`n[5/5] 后续步骤..."
Write-Success "`n✅ 所有依赖已安装成功！`n"

Write-Host "现在你可以运行以下命令:"
Write-Host "  pnpm test              # 运行所有测试（监听模式）" -ForegroundColor Green
Write-Host "  pnpm test:run          # 运行所有测试（一次）" -ForegroundColor Green
Write-Host "  pnpm test:ui           # 在 UI 中运行测试" -ForegroundColor Green
Write-Host "  pnpm test:coverage     # 生成覆盖率报告" -ForegroundColor Green

Write-Host "`n测试文件位置:"
Write-Host "  src/features/stats/__tests__/statsCalculator.test.ts" -ForegroundColor Cyan
Write-Host "  src/features/stats/__tests__/cacheStorage.test.ts" -ForegroundColor Cyan
Write-Host "  src/features/stats/__tests__/components.test.tsx" -ForegroundColor Cyan
Write-Host "  src/features/stats/__tests__/integration.test.ts" -ForegroundColor Cyan

Write-Host "`n文档:"
Write-Host "  STATS_TESTING_GUIDE.md  # 完整测试指南" -ForegroundColor Cyan

Write-Host "`n========================================`n" -ForegroundColor Cyan
