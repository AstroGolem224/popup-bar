import { render, screen } from '@testing-library/react';
import { SettingsPanel } from './SettingsPanel';
import '@testing-library/jest-dom';
import { useSettingsStore } from '../../stores/settingsStore';
import { vi, describe, beforeEach, it, expect } from 'vitest';

// Mock the tauri-bridge
vi.mock('../../utils/tauri-bridge', () => ({
  updateSettings: vi.fn().mockResolvedValue({}),
  setLaunchAtLogin: vi.fn().mockResolvedValue({}),
}));

describe('SettingsPanel', () => {
  beforeEach(() => {
    useSettingsStore.setState({
      settings: {
        hotzoneSize: 12,
        animationSpeed: 1.5,
        barWidthPx: 480,
        barHeightPx: 72,
        multiMonitor: false,
        autostart: false,
        tintColor: "rgba(0,0,0,0)",
        theme: "system",
      },
      setSettings: vi.fn(),
    });
  });

  it('renders hints for range sliders and has aria-modal', () => {
    render(<SettingsPanel />);

    // Check for aria-modal
    const dialog = screen.getByRole('dialog');
    expect(dialog).toHaveAttribute('aria-modal', 'true');

    // Check for the newly added hints
    expect(screen.getByText('12 px')).toBeInTheDocument();
    expect(screen.getByText('1.5x')).toBeInTheDocument();
  });
});
