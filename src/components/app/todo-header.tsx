import { Plus, Loader2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import { CardAction, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

import { ExternalApiToggle } from "./external-api-toggle";

type TodoHeaderProps = {
  isCreating: boolean;
  onCreateTodo: () => void;
  isServerRunning: boolean;
  isServerBusy: boolean;
  isPlatformSupported: boolean;
  statusMessage: string;
  onToggleApi: (nextEnabled: boolean) => void;
};

export function TodoHeader({
  isCreating,
  onCreateTodo,
  isServerRunning,
  isServerBusy,
  isPlatformSupported,
  statusMessage,
  onToggleApi,
}: TodoHeaderProps) {
  return (
    <CardHeader>
      <div>
        <CardTitle className="text-2xl font-semibold">待办清单</CardTitle>
        <CardDescription>使用宠物专注管理每日任务。</CardDescription>
      </div>
      <CardAction className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-end sm:gap-6">
        <ExternalApiToggle
          isRunning={isServerRunning}
          isBusy={isServerBusy}
          isPlatformSupported={isPlatformSupported}
          statusMessage={statusMessage}
          onToggle={onToggleApi}
        />
        <Button onClick={onCreateTodo} disabled={isCreating}>
          {isCreating ? (
            <Loader2 className="mr-2 size-4 animate-spin" aria-hidden="true" />
          ) : (
            <Plus className="mr-2 size-4" aria-hidden="true" />
          )}
          新建待办
        </Button>
      </CardAction>
    </CardHeader>
  );
}
