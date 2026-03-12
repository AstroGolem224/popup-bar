import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

/** Generic hook for invoking Tauri commands with loading/error state. */
export function useTauriCommand<TArgs extends Record<string, unknown>, TResult>(
  command: string,
) {
  const [data, setData] = useState<TResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const execute = useCallback(
    async (args?: TArgs): Promise<TResult | null> => {
      setLoading(true);
      setError(null);
      try {
        const result = await invoke<TResult>(command, args ?? {});
        setData(result);
        return result;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [command],
  );

  return { data, error, loading, execute };
}
