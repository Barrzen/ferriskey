<script lang="ts">
  import { page } from '$app/state';
  import { Bell, ChevronDown, Menu, Search, Sparkles } from 'lucide-svelte';
  import BrandMark from '$components/BrandMark.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { navigationGroups } from '$config/navigation';
  import { ripple } from '$utils/ripple';

  let {
    realm,
    children
  }: { realm: string; children: import('svelte').Snippet } = $props();

  let sidebarOpen = $state(false);

  const breadcrumbs = $derived.by(() =>
    page.url.pathname.split('/').filter(Boolean).slice(2)
  );

  function isActive(href: string) {
    return (
      page.url.pathname === href || page.url.pathname.startsWith(`${href}/`)
    );
  }
</script>

<div class="page-shell">
  <div class="app-shell glass-panel">
    {#if sidebarOpen}
      <button
        class="app-shell__backdrop"
        onclick={() => (sidebarOpen = false)}
        aria-label="Close navigation"
      ></button>
    {/if}

    <aside
      class:app-shell__sidebar--open={sidebarOpen}
      class="app-shell__sidebar"
    >
      <div class="app-shell__brand">
        <div class="app-shell__brand-row">
          <BrandMark />
          <div>
            <strong>FerrisKey</strong>
            <span>Dashboard preview</span>
          </div>
        </div>
        <button type="button" class="app-shell__realm-switch" use:ripple>
          <span>{realm}</span>
          <ChevronDown size={16} />
        </button>
      </div>

      <nav class="app-shell__nav" aria-label="Dashboard navigation">
        {#each navigationGroups as group}
          <section>
            <p>{group.title}</p>
            <div>
              {#each group.items as item}
                <a
                  href={item.href(realm)}
                  class:app-shell__link--active={isActive(item.href(realm))}
                  class="app-shell__link"
                  use:ripple
                >
                  <div class="app-shell__link-icon">
                    <item.icon size={18} />
                  </div>
                  <div>
                    <strong>{item.title}</strong>
                    <span>{item.description}</span>
                  </div>
                  {#if item.tag}
                    <small>{item.tag}</small>
                  {/if}
                </a>
              {/each}
            </div>
          </section>
        {/each}
      </nav>

      <div class="app-shell__promo">
        <div class="app-shell__promo-icon"><Sparkles size={18} /></div>
        <strong>Minimal-inspired rebuild</strong>
        <p>
          SSR-ready SvelteKit foundation with dark mode, ripple feedback, and
          realm-aware routing.
        </p>
      </div>
    </aside>

    <div class="app-shell__main">
      <header class="app-shell__header">
        <div class="app-shell__header-main">
          <button
            class="app-shell__menu"
            type="button"
            onclick={() => (sidebarOpen = true)}
            use:ripple
          >
            <Menu size={18} />
          </button>

          <div>
            <div class="app-shell__breadcrumbs">
              <span>Realms</span>
              {#each breadcrumbs as crumb}
                <span>/</span>
                <strong>{crumb.replaceAll('-', ' ')}</strong>
              {/each}
            </div>
            <h1>{breadcrumbs.at(-1)?.replaceAll('-', ' ') ?? 'Overview'}</h1>
          </div>
        </div>

        <div class="app-shell__header-actions">
          <label class="app-shell__search">
            <Search size={18} />
            <input
              type="search"
              placeholder="Search users, clients, flows..."
            />
          </label>
          <ThemeToggle />
          <button
            type="button"
            class="app-shell__icon-button"
            aria-label="Notifications"
            use:ripple
          >
            <Bell size={18} />
          </button>
        </div>
      </header>

      <main class="app-shell__content">{@render children()}</main>
    </div>
  </div>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-columns: 320px minmax(0, 1fr);
    min-height: calc(100vh - 48px);
    overflow: hidden;
  }

  .app-shell__sidebar {
    display: grid;
    gap: 28px;
    padding: 28px;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
  }

  .app-shell__brand,
  .app-shell__promo {
    display: grid;
    gap: 18px;
  }

  .app-shell__brand-row {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .app-shell__brand-row strong,
  .app-shell__link strong,
  .app-shell__promo strong {
    display: block;
  }

  .app-shell__brand-row span,
  .app-shell__link span,
  .app-shell__promo p,
  .app-shell__nav p,
  .app-shell__breadcrumbs {
    color: var(--text-muted);
  }

  .app-shell__realm-switch,
  .app-shell__icon-button,
  .app-shell__menu {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    cursor: pointer;
    box-shadow: var(--shadow-md);
  }

  .app-shell__realm-switch {
    justify-content: space-between;
    padding: 14px 16px;
    border-radius: 18px;
  }

  .app-shell__nav {
    display: grid;
    gap: 20px;
    align-content: start;
  }

  .app-shell__nav section {
    display: grid;
    gap: 12px;
  }

  .app-shell__nav p {
    margin: 0;
    padding-left: 12px;
    font-size: 0.82rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .app-shell__nav section div {
    display: grid;
    gap: 8px;
  }

  .app-shell__link {
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: 14px;
    align-items: center;
    padding: 14px;
    border-radius: 20px;
    border: 1px solid transparent;
    transition:
      transform 180ms ease,
      background 180ms ease,
      border-color 180ms ease;
  }

  .app-shell__link:hover {
    transform: translateX(2px);
    background: color-mix(in srgb, var(--surface) 72%, transparent);
    border-color: var(--border);
  }

  .app-shell__link--active {
    background: var(--surface);
    border-color: var(--border);
    box-shadow: var(--shadow-md);
  }

  .app-shell__link-icon,
  .app-shell__promo-icon,
  .app-shell__icon-button,
  .app-shell__menu {
    width: 42px;
    height: 42px;
    border-radius: 16px;
  }

  .app-shell__link-icon,
  .app-shell__promo-icon {
    display: grid;
    place-items: center;
    background: var(--bg-inset);
    color: var(--primary);
  }

  .app-shell__link span,
  .app-shell__promo p {
    font-size: 0.88rem;
    line-height: 1.55;
    margin: 4px 0 0;
  }

  .app-shell__link small {
    padding: 7px 10px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.75rem;
    font-weight: 700;
  }

  .app-shell__promo {
    padding: 20px;
    align-self: end;
    border-radius: 24px;
    background: linear-gradient(
      155deg,
      color-mix(in srgb, var(--primary) 14%, var(--surface)) 0%,
      var(--surface) 100%
    );
    border: 1px solid var(--border);
  }

  .app-shell__main {
    display: grid;
    grid-template-rows: auto 1fr;
    min-width: 0;
  }

  .app-shell__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
    padding: 28px 30px 16px;
  }

  .app-shell__header-main,
  .app-shell__header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .app-shell__breadcrumbs {
    display: flex;
    gap: 8px;
    font-size: 0.88rem;
    text-transform: capitalize;
  }

  h1 {
    margin: 8px 0 0;
    font: 700 clamp(1.8rem, 4vw, 2.6rem)/1 var(--font-display);
    letter-spacing: -0.05em;
    text-transform: capitalize;
  }

  .app-shell__search {
    display: flex;
    align-items: center;
    gap: 10px;
    width: min(320px, 42vw);
    padding: 13px 16px;
    border-radius: 18px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    color: var(--text-muted);
  }

  .app-shell__search input {
    width: 100%;
    border: 0;
    outline: 0;
    background: transparent;
    color: var(--text);
  }

  .app-shell__content {
    padding: 12px 30px 30px;
  }

  .app-shell__menu,
  .app-shell__backdrop {
    display: none;
  }

  .app-shell__backdrop {
    position: fixed;
    inset: 0;
    z-index: 39;
    background: rgba(2, 6, 23, 0.4);
    border: 0;
  }

  @media (max-width: 1200px) {
    .app-shell {
      grid-template-columns: 1fr;
    }

    .app-shell__sidebar {
      position: fixed;
      inset: 16px auto 16px 16px;
      width: min(320px, calc(100vw - 32px));
      z-index: 40;
      transform: translateX(-110%);
      transition: transform 220ms ease;
      border-radius: 32px;
      box-shadow: var(--shadow-lg);
    }

    .app-shell__sidebar--open {
      transform: translateX(0);
    }

    .app-shell__menu,
    .app-shell__backdrop {
      display: inline-flex;
    }
  }

  @media (max-width: 900px) {
    .app-shell__header,
    .app-shell__content {
      padding-left: 18px;
      padding-right: 18px;
    }

    .app-shell__header {
      align-items: flex-start;
      flex-direction: column;
    }

    .app-shell__header-actions {
      width: 100%;
      flex-wrap: wrap;
    }

    .app-shell__search {
      width: 100%;
    }
  }
</style>
