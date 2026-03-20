import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useHotzoneState } from "./useHotzoneState";
import { EVENTS } from "../types/events";
import {
  completeHideWindow,
  completeShowWindow,
  hideWindow,
  showWindow,
} from "../utils/tauri-bridge";

type EventHandler = () => void | Promise<void>;

const listeners = new Map<string, EventHandler>();

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async (eventName: string, handler: EventHandler) => {
    listeners.set(eventName, handler);
    return () => {
      listeners.delete(eventName);
    };
  }),
}));

vi.mock("../utils/tauri-bridge", () => ({
  showWindow: vi.fn(),
  completeShowWindow: vi.fn(),
  hideWindow: vi.fn(),
  completeHideWindow: vi.fn(),
}));

async function emit(eventName: string): Promise<void> {
  const handler = listeners.get(eventName);
  if (!handler) {
    throw new Error(`No listener registered for ${eventName}`);
  }

  await act(async () => {
    await handler();
  });
}

describe("useHotzoneState", () => {
  beforeEach(() => {
    listeners.clear();
    vi.clearAllMocks();
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
    });

    await emit(EVENTS.HOTZONE_ENTER);
    await emit(EVENTS.HOTZONE_LEAVE);

    await act(async () => {
      await result.current.onShelfAnimationEnd();
    });

    expect(completeShowWindow).not.toHaveBeenCalled();
    expect(completeHideWindow).toHaveBeenCalledWith(22);
  });
});
