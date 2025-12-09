#!/bin/bash

# 统计功能测试安装和运行脚本
# macOS/Linux 版本

echo "========================================"
echo "统计功能测试框架安装和运行"
echo "========================================"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 检查 pnpm 是否已安装
echo -e "\n${CYAN}[1/5] 检查 pnpm 是否已安装...${NC}"
if ! command -v pnpm &> /dev/null; then
    echo -e "${RED}❌ 未找到 pnpm。请先安装 pnpm:${NC}"
    echo -e "${YELLOW}  npm install -g pnpm${NC}"
    exit 1
fi
echo -e "${GREEN}✅ pnpm 已安装: $(which pnpm)${NC}"

# 检查 Node 版本
echo -e "\n${CYAN}[2/5] 检查 Node 版本...${NC}"
NODE_VERSION=$(node --version)
echo -e "${GREEN}✅ Node 版本: $NODE_VERSION${NC}"

# 安装测试依赖
echo -e "\n${CYAN}[3/5] 安装测试依赖...${NC}"
echo -e "${CYAN}运行: pnpm install -D vitest @testing-library/react @testing-library/user-event jsdom @vitest/ui${NC}"
pnpm install -D vitest @testing-library/react @testing-library/user-event jsdom @vitest/ui

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ 依赖安装失败${NC}"
    exit 1
fi
echo -e "${GREEN}✅ 依赖安装成功${NC}"

# 验证安装
echo -e "\n${CYAN}[4/5] 验证测试依赖安装...${NC}"
dependencies=("vitest" "@testing-library/react" "@testing-library/user-event" "jsdom")
all_installed=true

for dep in "${dependencies[@]}"; do
    if pnpm list "$dep" --depth=0 &>/dev/null; then
        echo -e "${GREEN}✅ $dep${NC}"
    else
        echo -e "${RED}❌ $dep 未安装${NC}"
        all_installed=false
    fi
done

if [ "$all_installed" = false ]; then
    echo -e "\n${RED}❌ 某些依赖未正确安装${NC}"
    echo -e "${YELLOW}请手动运行:${NC}"
    echo -e "${YELLOW}  pnpm install -D vitest @testing-library/react @testing-library/user-event jsdom @vitest/ui${NC}"
    exit 1
fi

# 提示后续步骤
echo -e "\n${CYAN}[5/5] 后续步骤...${NC}"
echo -e "\n${GREEN}✅ 所有依赖已安装成功!${NC}\n"

echo -e "${CYAN}现在你可以运行以下命令:${NC}"
echo -e "${GREEN}  pnpm test              # 运行所有测试（监听模式）${NC}"
echo -e "${GREEN}  pnpm test:run          # 运行所有测试（一次）${NC}"
echo -e "${GREEN}  pnpm test:ui           # 在 UI 中运行测试${NC}"
echo -e "${GREEN}  pnpm test:coverage     # 生成覆盖率报告${NC}"

echo -e "\n${CYAN}测试文件位置:${NC}"
echo -e "${CYAN}  src/features/stats/__tests__/statsCalculator.test.ts${NC}"
echo -e "${CYAN}  src/features/stats/__tests__/cacheStorage.test.ts${NC}"
echo -e "${CYAN}  src/features/stats/__tests__/components.test.tsx${NC}"
echo -e "${CYAN}  src/features/stats/__tests__/integration.test.ts${NC}"

echo -e "\n${CYAN}文档:${NC}"
echo -e "${CYAN}  STATS_TESTING_GUIDE.md  # 完整测试指南${NC}"

echo -e "\n========================================\n"
