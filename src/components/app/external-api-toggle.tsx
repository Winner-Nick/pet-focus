import { Checkbox } from "@/components/ui/checkbox";
import { Loader2 } from "lucide-react";

type ExternalApiToggleProps = {
  isRunning: boolean;
  isBusy: boolean;
  isPlatformSupported: boolean;
  statusMessage: string;
  onToggle: (nextEnabled: boolean) => void;
};

export function ExternalApiToggle({ isRunning, isBusy, isPlatformSupported, statusMessage, onToggle }: ExternalApiToggleProps) {
  const isDisabled = isBusy || !isPlatformSupported;

  return (
    <div className="flex items-start gap-3">
      <div className="mt-1">
        <Checkbox
          id="external-api-toggle"
          checked={isRunning}
          disabled={isDisabled}
          onCheckedChange={(checked) => {
            if (isDisabled) return;
            const nextEnabled = checked === true;
            if (nextEnabled === isRunning) return;
            onToggle(nextEnabled);
          }}
          aria-label="切换外部 REST API"
        />
      </div>
      <div>
        <label htmlFor="external-api-toggle" className="text-sm font-medium leading-none">
          外部 REST API
        </label>
        <p className="mt-1 text-xs text-muted-foreground">{statusMessage}</p>
      </div>
      {isBusy && <Loader2 className="mt-1 size-4 animate-spin text-muted-foreground" aria-hidden="true" />}
    </div>
  );
}
