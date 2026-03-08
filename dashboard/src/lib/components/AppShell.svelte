<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  import {
    Bell,
    ChevronDown,
    LogOut,
    Menu,
    Search
  } from 'lucide-svelte';
  import BrandLogo from '$components/BrandLogo.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { navigationGroups } from '$config/navigation';
  import { sanitizeCurrentAppPath } from '$utils/auth-redirect';
  import { ripple } from '$utils/ripple';
  import type { SessionUser } from '$lib/auth/session';

  let {
    realm,
    realms = [],
    user,
    children
  }: {
    realm: string;
    realms: { id: string; name: string }[];
    user: SessionUser;
    children: import('svelte').Snippet;
  } = $props();

  let sidebarOpen = $state(false);
  let realmDropdownOpen = $state(false);

  const breadcrumbs = $derived.by(() =>
    page.url.pathname
      .split('/')
      .filter(Boolean)
      .slice(2)
      .map((crumb) => crumb.replaceAll('-', ' '))
  );

  const activeTitle = $derived(breadcrumbs.at(-1) ?? 'overview');

  const userInitials = $derived.by(() =>
    user.name
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? '')
      .join('') || 'BA'
  );

  const userMeta = $derived(user.preferredUsername ?? user.email ?? realm);
  const sanitizedCurrentPath = $derived(sanitizeCurrentAppPath(page.url));
  const loginHref = $derived(
    `/realms/${realm}/authentication/login?next=${encodeURIComponent(sanitizedCurrentPath)}`
  );
  const logoutHref = $derived.by(() => {
    const target = new URL(
      `${page.url.origin}/api/realms/${realm}/protocol/openid-connect/logout`
    );

    target.searchParams.set('client_id', 'security-admin-console');
    target.searchParams.set(
      'post_logout_redirect_uri',
      `${page.url.origin}/realms/${realm}/authentication/login`
    );

    return target.toString();
  });

  function isActive(href: string) {
    return (
      page.url.pathname === href || page.url.pathname.startsWith(`${href}/`)
    );
  }

  onMount(() => {
    if (!browser || !user.expiresAt) {
      return;
    }

    const remaining = user.expiresAt - Date.now();

    if (remaining <= 0) {
      void goto(loginHref, { replaceState: true });
      return;
    }

    const timeout = window.setTimeout(() => {
      void goto(loginHref, { replaceState: true });
    }, remaining);

    return () => window.clearTimeout(timeout);
  });
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
      <div class="app-shell__brand-block">
        <BrandLogo showLabel={false} />
        <button type="button" class="app-shell__realm-switch" use:ripple onclick={() => (realmDropdownOpen = !realmDropdownOpen)}>
          <div>
            <small>Realm</small>
            <strong>{realm}</strong>
          </div>
          <ChevronDown size={16} />
        </button>
        {#if realmDropdownOpen}
          <div class="app-shell__realm-dropdown">
            {#each realms as r (r.name)}
              <a
                href="/realms/{r.name}/overview"
                class="app-shell__realm-option"
                class:app-shell__realm-option--active={r.name === realm}
                onclick={() => (realmDropdownOpen = false)}
              >
                <strong>{r.name}</strong>
              </a>
            {/each}
            <a
              href="/realms/{realm}/realms"
              class="app-shell__realm-option app-shell__realm-option--manage"
              onclick={() => (realmDropdownOpen = false)}
            >
              <strong>Manage realms</strong>
            </a>
          </div>
        {/if}
      </div>

      <nav class="app-shell__nav" aria-label="Dashboard navigation">
        {#each navigationGroups as group (group.title)}
          <section>
            <p>{group.title}</p>
            <div>
              {#each group.items as item (item.title)}
                <a
                  href={item.href(realm)}
                  class:app-shell__link--active={isActive(item.href(realm))}
                  class="app-shell__link"
                  use:ripple
                >
                  <div class="app-shell__link-icon">
                    <item.icon size={18} />
                  </div>
                  <div class="app-shell__link-copy">
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
              <span>dashboard</span>
              {#each breadcrumbs as crumb, index (crumb + index)}
                <span>/</span>
                <strong>{crumb}</strong>
              {/each}
            </div>
            <h1>{activeTitle}</h1>
            <p>
              FerrisKey admin workspace for users, clients, roles, and realm
              operations.
            </p>
          </div>
        </div>

        <div class="app-shell__header-actions">
          <label class="app-shell__search">
            <Search size={18} />
            <input
              type="search"
              placeholder="Search users, clients, events..."
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
          <a href={logoutHref} class="app-shell__icon-button" aria-label="Sign out" use:ripple>
            <LogOut size={18} />
          </a>
          <button type="button" class="app-shell__profile" use:ripple>
            <span class="app-shell__avatar">{userInitials}</span>
            <div>
              <strong>{user.name}</strong>
              <small>{userMeta}</small>
            </div>
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
    min-height: calc(100vh - 32px);
    overflow: hidden;
  }

  .app-shell__sidebar {
    display: grid;
    align-content: start;
    gap: 22px;
    padding: 20px;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
  }

  .app-shell__brand-block {
    display: grid;
    gap: 14px;
  }

  .app-shell__brand-block {
    position: relative;
  }

  .app-shell__realm-switch,
  .app-shell__menu,
  .app-shell__icon-button,
  .app-shell__profile {
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

  .app-shell__realm-dropdown {
    position: absolute;
    top: calc(100% + 6px);
    left: 0; right: 0;
    z-index: 50;
    display: grid; gap: 4px;
    padding: 8px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-lg);
  }

  .app-shell__realm-option {
    display: block;
    padding: 10px 14px;
    border-radius: 12px;
    text-decoration: none;
    color: var(--text-muted);
    font-weight: 600;
    transition: background 120ms ease;
  }
  .app-shell__realm-option:hover { background: var(--bg-inset); color: var(--text); }
  .app-shell__realm-option--active { background: var(--primary-soft); color: var(--primary); }
  .app-shell__realm-option--manage {
    margin-top: 6px;
    padding-top: 16px;
    border-top: 1px solid var(--border);
  }
  .app-shell__realm-option strong { color: inherit; }

  .app-shell__realm-switch small,
  .app-shell__breadcrumbs,
  .app-shell__header-main p,
  .app-shell__search,
  .app-shell__link span,
  .app-shell__nav p,
  .app-shell__profile small {
    color: var(--text-muted);
  }

  .app-shell__link-icon,
  .app-shell__avatar,
  .app-shell__icon-button,
  .app-shell__menu {
    display: grid;
    place-items: center;
    width: 42px;
    height: 42px;
    border-radius: 14px;
  }

  .app-shell__link-icon,
  .app-shell__avatar {
    background: var(--primary-soft);
    color: var(--primary);
    font-weight: 800;
  }

  .app-shell__link strong,
  .app-shell__profile strong {
    display: block;
  }

  .app-shell__link span {
    margin: 4px 0 0;
    font-size: 0.86rem;
    line-height: 1.6;
  }

  .app-shell__nav {
    display: grid;
    gap: 18px;
  }

  .app-shell__nav section {
    display: grid;
    gap: 12px;
  }

  .app-shell__nav p {
    margin: 0;
    padding-left: 10px;
    font-size: 0.78rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .app-shell__nav section > div {
    display: grid;
    gap: 6px;
  }

  .app-shell__link {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 16px;
    border: 1px solid transparent;
    transition:
      background 180ms ease,
      transform 180ms ease,
      border-color 180ms ease;
  }

  .app-shell__link:hover {
    background: var(--surface);
    border-color: var(--border);
    transform: translateX(2px);
  }

  .app-shell__link--active {
    background: var(--surface);
    border-color: var(--border);
    box-shadow: var(--shadow-md);
  }

  .app-shell__link-copy {
    min-width: 0;
  }

  .app-shell__link small {
    font-size: 0.76rem;
    font-weight: 700;
  }

  .app-shell__link small {
    padding: 6px 10px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
  }

  .app-shell__main {
    display: grid;
    grid-template-rows: auto 1fr;
    min-width: 0;
  }

  .app-shell__header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 20px;
    padding: 24px 28px 14px;
  }

  .app-shell__header-main,
  .app-shell__header-actions {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .app-shell__breadcrumbs {
    display: flex;
    gap: 8px;
    font-size: 0.82rem;
    text-transform: capitalize;
  }

  h1 {
    margin: 8px 0 4px;
    font: 700 clamp(1.9rem, 4vw, 2.6rem)/1 var(--font-display);
    letter-spacing: -0.05em;
    text-transform: capitalize;
  }

  .app-shell__header-main p {
    margin: 0;
    font-size: 0.92rem;
  }

  .app-shell__search {
    display: flex;
    align-items: center;
    gap: 10px;
    width: min(340px, 42vw);
    padding: 13px 16px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
  }

  .app-shell__search input {
    width: 100%;
    border: 0;
    background: transparent;
    color: var(--text);
    outline: 0;
  }

  .app-shell__profile {
    padding: 6px 14px 6px 6px;
    border-radius: 16px;
  }

  .app-shell__profile div {
    display: grid;
    text-align: left;
  }

  .app-shell__content {
    padding: 10px 28px 28px;
  }

  .app-shell__menu,
  .app-shell__backdrop {
    display: none;
  }

  .app-shell__backdrop {
    position: fixed;
    inset: 0;
    z-index: 39;
    background: rgba(22, 28, 36, 0.48);
    border: 0;
  }

  @media (max-width: 1200px) {
    .app-shell {
      grid-template-columns: 1fr;
    }

    .app-shell__sidebar {
      position: fixed;
      inset: 12px auto 12px 12px;
      width: min(320px, calc(100vw - 24px));
      z-index: 40;
      transform: translateX(-110%);
      transition: transform 220ms ease;
      border-radius: 24px;
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
      padding-left: 16px;
      padding-right: 16px;
    }

    .app-shell__header {
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
