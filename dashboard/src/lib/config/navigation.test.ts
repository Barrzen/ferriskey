import { describe, expect, it } from 'vitest';
import { navigationGroups } from '$lib/config/navigation';

describe('navigationGroups', () => {
  it('builds realm-aware links', () => {
    const firstGroup = navigationGroups[0];
    const firstItem = firstGroup?.items[0];

    expect(firstItem?.href('master')).toBe('/realms/master/overview');
    expect(firstItem?.href('engineering')).toBe('/realms/engineering/overview');
  });

  it('keeps titles unique within each group', () => {
    for (const group of navigationGroups) {
      const titles = group.items.map((item) => item.title);
      expect(new Set(titles).size).toBe(titles.length);
    }
  });
});
