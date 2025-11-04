/**
 * UI 组件统一导出
 *
 * 本目录下的所有组件是基础 UI 组件（Tier 1），来自 shadcn UI 库或自定义原语。
 * 这些组件完全无状态，不含业务逻辑，通过 Props 接收所有配置。
 *
 * 使用规范：
 * - 从 @/components/ui 导入
 * - 避免深层导入（如 @/components/ui/button）
 * - 组件只负责展示，不处理业务逻辑
 */

// Form components
export { Button, buttonVariants } from "./button";

export { Input } from "./input";

export { Label } from "./label";

export { Checkbox } from "./checkbox";

export { RadioGroup, RadioGroupItem } from "./radio-group";

export { Textarea } from "./textarea";

// Container components
export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardDescription,
  CardContent,
  CardAction,
} from "./card";

// Structural components
export { Separator } from "./separator";

// Navigation
export { Tabs, TabsList, TabsTrigger, TabsContent } from "./tabs";

// Progress
export { Progress } from "./progress";

// Toast/Notification
export { Toaster } from "./sonner";
