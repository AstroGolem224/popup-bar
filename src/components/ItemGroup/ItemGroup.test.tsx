import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { ItemGroup } from './ItemGroup';
import { vi } from 'vitest';

describe('ItemGroup', () => {
  it('renders specific ARIA labels for color picker buttons', () => {
    const mockGroup = { id: 'g1', name: 'My Group', itemType: 'group' as const, path: '', iconCacheKey: '', children: [] };
    const onUpdateGroup = vi.fn();

    render(
      <ItemGroup
        group={mockGroup}
        items={[]}
        onUpdateGroup={onUpdateGroup}
      />
    );

    // Verify all 5 color buttons are present with distinct aria-labels
    const whiteBtn = screen.getByLabelText('Farbe ändern: Weiß/Standard');
    expect(whiteBtn).toBeDefined();

    const blueBtn = screen.getByLabelText('Farbe ändern: Blau');
    expect(blueBtn).toBeDefined();

    const greenBtn = screen.getByLabelText('Farbe ändern: Grün');
    expect(greenBtn).toBeDefined();

    const orangeBtn = screen.getByLabelText('Farbe ändern: Orange');
    expect(orangeBtn).toBeDefined();

    const pinkBtn = screen.getByLabelText('Farbe ändern: Pink');
    expect(pinkBtn).toBeDefined();

    // Verify one of them fires correctly
    fireEvent.click(blueBtn);
    expect(onUpdateGroup).toHaveBeenCalledWith(expect.objectContaining({
      id: 'g1',
      color: 'rgba(100,180,255,0.6)'
    }));
  });
});
