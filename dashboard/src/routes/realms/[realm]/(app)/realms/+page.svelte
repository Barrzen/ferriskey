<script lang="ts">
  import type { ActionData, PageData } from './$types';
  import {
    Building2,
    FolderTree,
    Globe,
    Pencil,
    Plus,
    Search,
    ShieldCheck,
    Trash2
  } from 'lucide-svelte';
  import { enhance } from '$app/forms';
  import { page } from '$app/state';
  import MetricCard from '$components/MetricCard.svelte';
  import SectionCard from '$components/SectionCard.svelte';
  import Dialog from '$components/Dialog.svelte';
  import { showToast } from '$lib/stores/toast';
  import { ripple } from '$utils/ripple';

  let { data, form }: { data: PageData; form: ActionData } = $props();

  const currentRealm = $derived(String(page.params.realm ?? 'master'));
  let searchTerm = $state('');
  let showCreate = $state(false);
  let creating = $state(false);
  let showRename = $state(false);
  let renaming = $state(false);
  let renameTarget = $state<PageData['realms'][number] | null>(null);
  let showDelete = $state(false);
  let deleting = $state(false);
  let deleteTarget = $state<PageData['realms'][number] | null>(null);

  $effect(() => {
    if (form && typeof form === 'object') {
      const payload = form as Record<string, unknown>;
      if (payload.success && typeof payload.message === 'string') {
        showToast(payload.message, 'success');
        showCreate = false;
        showRename = false;
        showDelete = false;
      } else if (typeof payload.error === 'string') {
        showToast(payload.error, 'error');
      }

      creating = false;
      renaming = false;
      deleting = false;
    }
  });

  function formatDate(value: string) {
    const date = new Date(value);
    if (Number.isNaN(date.getTime())) return value;
    return new Intl.DateTimeFormat('en', {
      day: '2-digit',
      month: 'short',
      year: 'numeric'
    }).format(date);
  }

  const visibleRealms = $derived.by(() => {
    const term = searchTerm.trim().toLowerCase();
    if (!term) return data.realms;
    return data.realms.filter((realm) => realm.name.toLowerCase().includes(term));
  });
</script>

<div class="realms-page">
  <section class="realms-page__hero glass-panel">
    <div>
      <p>Realm operations</p>
      <h2>Switch and manage tenant boundaries.</h2>
      <span>
        Review accessible realms, create a new realm, rename an existing one,
        or switch into another environment from a single page.
      </span>
    </div>
    <div class="realms-page__hero-actions">
      <button type="button" class="realms-page__cta" use:ripple onclick={() => (showCreate = true)}>
        <Plus size={16} />
        Create realm
      </button>
    </div>
  </section>

  <section class="realms-page__metrics">
    <MetricCard title="Accessible realms" value={String(data.realms.length)} delta={`${visibleRealms.length} visible`} meta="realms available to the current user">
      {#snippet icon()}<FolderTree size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard title="Current realm" value={currentRealm} delta="active context" meta="used for auth, navigation, and scoped resources" tone="success">
      {#snippet icon()}<ShieldCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard title="Switch targets" value={String(Math.max(data.realms.length - 1, 0))} delta="other environments" meta="available directly from the sidebar switcher" tone="primary">
      {#snippet icon()}<Globe size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <SectionCard eyebrow="Directory" title="Realms" description="View, switch, rename, or delete accessible realms.">
    {#snippet actions()}
      <label class="realms-page__search">
        <Search size={16} />
        <input type="search" placeholder="Search realms..." bind:value={searchTerm} />
      </label>
    {/snippet}

    <div class="realms-page__list">
      {#if visibleRealms.length > 0}
        {#each visibleRealms as realm (realm.id)}
          <article class="realm-row" class:realm-row--active={realm.name === currentRealm}>
            <div class="realm-row__icon">
              <Building2 size={18} />
            </div>
            <div class="realm-row__info">
              <div class="realm-row__heading">
                <strong>{realm.name}</strong>
                {#if realm.name === currentRealm}
                  <span>Current</span>
                {/if}
              </div>
              <small>Created {formatDate(realm.created_at)} · Updated {formatDate(realm.updated_at)}</small>
            </div>
            <div class="realm-row__actions">
              <a href="/realms/{realm.name}/overview" class="realm-row__switch" use:ripple>
                Open
              </a>
              <button
                type="button"
                aria-label="Rename realm"
                use:ripple
                onclick={() => {
                  renameTarget = realm;
                  showRename = true;
                }}
              >
                <Pencil size={14} />
              </button>
              <button
                type="button"
                aria-label="Delete realm"
                use:ripple
                disabled={realm.name === currentRealm}
                onclick={() => {
                  deleteTarget = realm;
                  showDelete = true;
                }}
              >
                <Trash2 size={14} />
              </button>
            </div>
          </article>
        {/each}
      {:else}
        <div class="realms-page__empty">
          <strong>No realms match this filter.</strong>
          <small>Try a broader search term or create a new realm.</small>
        </div>
      {/if}
    </div>
  </SectionCard>
</div>

<Dialog bind:open={showCreate} title="Create realm">
  <form
    method="POST"
    action="?/create"
    class="realm-form"
    use:enhance={() => {
      creating = true;
      return async ({ update }) => {
        await update();
      };
    }}
  >
    <label>
      <span>Realm name</span>
      <input type="text" name="name" placeholder="engineering" required />
    </label>
    <button type="submit" class="realm-form__submit" use:ripple disabled={creating}>
      {creating ? 'Creating...' : 'Create realm'}
    </button>
  </form>
</Dialog>

<Dialog bind:open={showRename} title="Rename realm">
  <form
    method="POST"
    action="?/rename"
    class="realm-form"
    use:enhance={() => {
      renaming = true;
      return async ({ update }) => {
        await update();
      };
    }}
  >
    <input type="hidden" name="realm_name" value={renameTarget?.name ?? ''} />
    <label>
      <span>New realm name</span>
      <input type="text" name="name" value={renameTarget?.name ?? ''} required />
    </label>
    <button type="submit" class="realm-form__submit" use:ripple disabled={renaming}>
      {renaming ? 'Saving...' : 'Save realm'}
    </button>
  </form>
</Dialog>

<Dialog bind:open={showDelete} title="Delete realm">
  <form
    method="POST"
    action="?/delete"
    class="realm-form"
    use:enhance={() => {
      deleting = true;
      return async ({ update }) => {
        await update();
      };
    }}
  >
    <input type="hidden" name="realm_name" value={deleteTarget?.name ?? ''} />
    <p class="realm-form__danger-copy">
      Delete <strong>{deleteTarget?.name}</strong>? This removes the realm and its
      associated data permanently.
    </p>
    <button type="submit" class="realm-form__submit realm-form__submit--danger" use:ripple disabled={deleting}>
      {deleting ? 'Deleting...' : 'Delete realm'}
    </button>
  </form>
</Dialog>

<style>
  .realms-page {
    display: grid;
    gap: 24px;
  }

  .realms-page__hero {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 20px;
    padding: 24px;
  }

  .realms-page__hero p,
  .realms-page__hero span,
  .realm-row__info small,
  .realms-page__empty small,
  .realm-form label span,
  .realm-form__danger-copy {
    color: var(--text-muted);
  }

  .realms-page__hero p {
    margin: 0 0 10px;
    font-size: 0.82rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .realms-page__hero h2 {
    margin: 0;
    font: 700 clamp(1.8rem, 4vw, 2.8rem)/0.95 var(--font-display);
    letter-spacing: -0.05em;
  }

  .realms-page__hero span {
    display: block;
    margin-top: 12px;
    max-width: 720px;
    line-height: 1.7;
  }

  .realms-page__hero-actions {
    display: flex;
    justify-content: flex-end;
    width: 100%;
  }

  .realms-page__cta,
  .realm-row__switch,
  .realm-row__actions button,
  .realm-form__submit {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    border: 0;
    cursor: pointer;
    font-weight: 700;
  }

  .realms-page__cta,
  .realm-form__submit {
    padding: 13px 18px;
    border-radius: 16px;
    background: var(--primary);
    color: white;
  }

  .realms-page__metrics {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 18px;
  }

  .realms-page__search {
    display: flex;
    align-items: center;
    gap: 10px;
    width: min(320px, 100%);
    padding: 12px 14px;
    border-radius: 16px;
    border: 1px solid var(--border);
    background: var(--surface);
  }

  .realms-page__search input,
  .realm-form input {
    width: 100%;
    border: 0;
    outline: 0;
    background: transparent;
    color: var(--text);
    font: inherit;
  }

  .realms-page__list,
  .realm-form {
    display: grid;
    gap: 12px;
  }

  .realm-row {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    gap: 16px;
    padding: 16px 18px;
    border-radius: 18px;
    border: 1px solid var(--border);
    background: var(--surface);
  }

  .realm-row--active {
    border-color: color-mix(in srgb, var(--primary) 36%, var(--border));
    box-shadow: var(--shadow-md);
  }

  .realm-row__icon {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    border-radius: 14px;
    background: var(--primary-soft);
    color: var(--primary);
  }

  .realm-row__info,
  .realm-row__heading {
    display: grid;
    gap: 6px;
  }

  .realm-row__heading {
    grid-auto-flow: column;
    justify-content: start;
    align-items: center;
    gap: 10px;
  }

  .realm-row__heading span {
    padding: 5px 10px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.76rem;
    font-weight: 700;
  }

  .realm-row__actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .realm-row__switch {
    padding: 10px 14px;
    border-radius: 12px;
    background: var(--bg-inset);
    color: var(--text);
    text-decoration: none;
  }

  .realm-row__actions button {
    width: 38px;
    height: 38px;
    border-radius: 12px;
    background: var(--bg-inset);
    color: var(--text);
  }

  .realm-row__actions button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .realms-page__empty {
    padding: 24px;
    border-radius: 18px;
    border: 1px dashed var(--border);
    background: var(--bg-inset);
  }

  .realms-page__empty strong,
  .realm-row__info strong {
    display: block;
  }

  .realm-form label {
    display: grid;
    gap: 8px;
  }

  .realm-form input {
    padding: 13px 14px;
    border-radius: 14px;
    border: 1px solid var(--border);
    background: var(--surface);
  }

  .realm-form__danger-copy {
    margin: 0;
    line-height: 1.7;
  }

  .realm-form__submit--danger {
    background: var(--danger);
  }

  @media (max-width: 1100px) {
    .realms-page__metrics {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 900px) {
    .realms-page__hero,
    .realm-row {
      grid-template-columns: 1fr;
    }

    .realms-page__hero {
      align-items: stretch;
    }

    .realms-page__hero-actions,
    .realm-row__actions {
      justify-content: flex-start;
    }
  }
</style>
