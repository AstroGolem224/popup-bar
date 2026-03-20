import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useHotzoneState } from "./useHotzoneState";
import { EVENTS } from "../types/events";
import {
  completeHideWindow,
  completeShowWindow,
  getCurrentWindow,
  hideWindow,
  listen,
  showWindow,
} from "../utils/tauri-bridge";

type EventPayload = { edge?: string };
type EventHandler = (event: { payload: EventPayload }) => void | Promise<void>;

const listeners = new Map<string, EventHandler>();

vi.mock("../utils/tauri-bridge", () => ({
  listen: vi.fn(async (eventName: string, handler: EventHandler) => {
    listeners.set(eventName, handler);
    return () => {
      listeners.delete(eventName);
    };
  }),
  getCurrentWindow: vi.fn(() => ({ label: "main" })),
  showWindow: vi.fn(),
  completeShowWindow: vi.fn(),
  hideWindow: vi.fn(),
  completeHideWindow: vi.fn(),
}));

async function emit(eventName: string, payload: EventPayload = { edge: "top" }): Promise<void> {
  const handler = listeners.get(eventName);
  if (!handler) {
    throw new Error(`No listener registered for ${eventName}`);
  }

  await act(async () => {
    await handler({ payload });
  });
}

describe("useHotzoneState", () => {
  beforeEach(() => {
    listeners.clear();
    vi.clearAllMocks();
    vi.mocked(listen).mockClear();
    vi.mocked(getCurrentWindow).mockReturnValue({ label: "main" } as ReturnType<typeof getCurrentWindow>);
    vi.mocked(showWindow).mockResolvedValue(11);
    vi.mocked(hideWindow).mockResolvedValue(22);
    vi.mocked(completeShowWindow).mockResolvedValue(true);
    vi.mocked(completeHideWindow).mockResolvedValue(true);
  });

  it("sets visible and completes show lifecycle on enter", async () => {
    const { result } = renderHook(() => useHotzoneState());

    await waitFor(() => {
      expect(listeners.has(EVENTS.HOTZONE_ENTER)).toBe(true);
      expect(listeners.has(EVENTS.HOTZONE_LEAVE)).toBe(true);
      expect(listeners.has(EVENTS.TOGGLE_VISIBILITY)).toBe(true);
    });

    expect(result.current.isVisible).toBe(false);

    await emit(EVENTS.HOTZONE_ENTER);

    expect(result.current.isVisible).toBe(true);
    expect(showWindow).toHaveBeenCalledTimes(1);

    await act(async () => {
      await result.current.onShelfAnimationEnd();
    });

    expect(completeShowWindow).toHaveBeenCalledWith(11);
    expect(completeHideWindow).not.toHaveBeenCalled();
  });

  it("sets hidden and completes hide lifecycle on leave", async () => {
    const { result } = renderHook(() => useHotzoneState());

    await waitFor(() => {
      expect(listeners.has(EVENTS.HOTZONE_ENTER)).toBe(true);
      expect(listeners.has(EVENTS.HOTZONE_LEAVE)).toBe(true);
      expect(listeners.has(EVENTS.TOGGLE_VISIBILITY)).toBe(true);
    });

    await emit(EVENTS.HOTZONE_LEAVE);
    expect(result.current.isVisible).toBe(false);
    expect(hideWindow).toHaveBeenCalledTimes(1);

    await act(async () => {
      await result.current.onShelfAnimationEnd();
    });

    expect(completeHideWindow).toHaveBeenCalledWith(22);
    expect(completeShowWindow).not.toHaveBeenCalled();
  });

  it("ignores stale show token after quick leave", async () => {
    const { result } = renderHook(() => useHotzoneState());

    await waitFor(() => {
      expect(listeners.has(EVENTS.HOTZONE_ENTER)).toBe(true);
      expect(listeners.has(EVENTS.HOTZONE_LEAVE)).toBe(true);
      expect(listeners.has(EVENTS.TOGGLE_VISIBILITY)).toBe(true);
    });

    await emit(EVENTS.HOTZONE_ENTER);
    await emit(EVENTS.HOTZONE_LEAVE);

    await act(async () => {
      await result.current.onShelfAnimationEnd();
    });

    expect(completeShowWindow).not.toHaveBeenCalled();
    expect(completeHideWindow).toHaveBeenCalledWith(22);
  });

  it("toggles visibility for the main window via system event", async () => {
    const { result } = renderHook(() => useHotzoneState());

    await waitFor(() => {
      expect(listeners.has(EVENTS.TOGGLE_VISIBILITY)).toBe(true);
    });

    await emit(EVENTS.TOGGLE_VISIBILITY, {});
    expect(result.current.isVisible).toBe(true);
    expect(showWindow).toHaveBeenCalledTimes(1);

    await emit(EVENTS.TOGGLE_VISIBILITY, {});
    expect(result.current.isVisible).toBe(false);
    expect(hideWindow).toHaveBeenCalledTimes(1);
  });
});
