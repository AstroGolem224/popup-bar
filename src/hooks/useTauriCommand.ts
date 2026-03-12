/**
 * Hook: Generic Tauri command invoker.
 *
 * Provides a typed wrapper around Tauri's invoke API with
 * loading state and error handling.
 *
 * Phase 0: Returns a stub invoker that logs to console.
 * Phase 1+: Wired to @tauri-apps/api invoke.
 */

interface UseTauriCommandReturn<T> {
  /** Execute a Tauri command by name. */
  invoke: (command: string, args?: Record<string, unknown>) => Promise<T | null>;
  /** Whether a command is currently in flight. */
  isLoading: boolean;
  /** Last error message, if any. */
  error: string | null;
}

export function useTauriCommand<T = unknown>(): UseTauriCommandReturn<T> {
  return {
    invoke: async (command: string, args?: Record<string, unknown>): Promise<T | null> => {
      console.warn(`Tauri invoke stub: ${command}`, args);
      return null;
    },
    isLoading: false,
    error: null,
  };
}
