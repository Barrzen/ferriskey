<script lang="ts">
  import { MonitorCog, MoonStar, SunMedium } from 'lucide-svelte';
  import { ripple } from '$lib/utils/ripple';

  type ThemeMode = 'light' | 'dark' | 'system';

  const themes: { value: ThemeMode; label: string; icon: typeof SunMedium }[] =
    [
      { value: 'light', label: 'Light mode', icon: SunMedium },
      { value: 'dark', label: 'Dark mode', icon: MoonStar },
      { value: 'system', label: 'System theme', icon: MonitorCog }
    ];

  let theme = $state<ThemeMode>('system');
  const THEME_KEY = 'barrzen-dashboard-theme';

  function applyTheme(nextTheme: ThemeMode) {
    theme = nextTheme;
    localStorage.setItem(THEME_KEY, nextTheme);

    const resolved =
      nextTheme === 'system'
        ? window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
          : 'light'
        : nextTheme;

    document.documentElement.dataset.theme = resolved;
  }

  $effect(() => {
    const storedTheme = localStorage.getItem(THEME_KEY) as ThemeMode | null;
    applyTheme(storedTheme ?? 'system');

    const media = window.matchMedia('(prefers-color-scheme: dark)');
    const onChange = () => {
      if (theme === 'system') {
        applyTheme('system');
      }
    };

    media.addEventListener('change', onChange);
    return () => media.removeEventListener('change', onChange);
  });
</script>

<div class="theme-toggle" role="radiogroup" aria-label="Theme mode">
  {#each themes as option}
    <button
      type="button"
      class:theme-toggle__button--active={theme === option.value}
      class="theme-toggle__button"
      aria-label={option.label}
      aria-pressed={theme === option.value}
      onclick={() => applyTheme(option.value)}
      use:ripple={{ centered: true }}
    >
      <option.icon size={16} strokeWidth={2.2} />
    </button>
  {/each}
</div>

<style>
  .theme-toggle {
    display: inline-flex;
    gap: 6px;
    padding: 6px;
    border-radius: 999px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .theme-toggle__button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 38px;
    height: 38px;
    border: 0;
    border-radius: 999px;
    background: transparent;
    color: var(--text-soft);
    cursor: pointer;
    transition:
      background 180ms ease,
      color 180ms ease,
      transform 180ms ease;
  }

  .theme-toggle__button:hover {
    transform: translateY(-1px);
    color: var(--text);
  }

  .theme-toggle__button--active {
    background: var(--surface);
    color: var(--primary);
    box-shadow: var(--shadow-md);
  }
</style>
