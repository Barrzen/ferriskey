<script lang="ts">
  import type { PageData } from './$types';
  import { BadgeCheck, Shield, Users } from 'lucide-svelte';
  import ChipTabs from '$components/ChipTabs.svelte';
  import LinearMeter from '$components/LinearMeter.svelte';
  import MetricCard from '$components/MetricCard.svelte';
  import SectionCard from '$components/SectionCard.svelte';

  let { data }: { data: PageData } = $props();
  const roleTabs = ['Realm roles', 'Client roles', 'Permission sets'];
  let activeTab = $state('Realm roles');

  const assignmentCounts = $derived.by(() => {
    const counts = new Map<string, number>();

    for (const user of data.users) {
      for (const role of user.roles ?? []) {
        counts.set(role.id, (counts.get(role.id) ?? 0) + 1);
      }
    }

    return counts;
  });

  const visibleRoles = $derived.by(() =>
    data.roles.filter((role) => {
      if (activeTab === 'Realm roles') {
        return !role.client_id;
      }

      if (activeTab === 'Client roles') {
        return Boolean(role.client_id);
      }

      return role.permissions.length > 0;
    })
  );

  const selectedRole = $derived(visibleRoles[0] ?? data.roles[0] ?? null);
  const assignedUsers = $derived(
    data.users.filter((user) => (user.roles?.length ?? 0) > 0).length
  );
  const realmRoles = $derived(data.roles.filter((role) => !role.client_id).length);
  const permissionCoverage = $derived(
    data.roles.length === 0
      ? 0
      : Math.round(
          (data.roles.filter((role) => role.permissions.length > 0).length / data.roles.length) *
            100
        )
  );

  function roleScope(role: PageData['roles'][number]) {
    return role.client_id ? 'Client' : 'Realm';
  }

  function roleDescription(role: PageData['roles'][number]) {
    return role.description ?? `${role.permissions.length} mapped permission(s)`;
  }

  function roleAssignments(role: PageData['roles'][number]) {
    const count = assignmentCounts.get(role.id) ?? 0;
    return `${count} assignee${count === 1 ? '' : 's'}`;
  }
</script>

<div class="roles-page">
  <section class="roles-page__hero glass-panel">
    <div>
      <p>Roles workspace</p>
      <h2>Permission modeling without the old clutter.</h2>
      <span
        >Move between realm roles, client roles, and permission coverage with
        cleaner segmentation and stronger context.</span
      >
    </div>
    <ChipTabs items={roleTabs} active={activeTab} tone="soft" onselect={(item) => (activeTab = item)} />
  </section>

  <section class="roles-page__metrics">
    <MetricCard
      title="Total roles"
      value={String(data.roles.length)}
      delta={`${realmRoles} realm roles`}
      meta="live realm and client scopes"
    >
      {#snippet icon()}<BadgeCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Assigned users"
      value={String(assignedUsers)}
      delta={`${data.users.length - assignedUsers} without roles`}
      meta="identities with at least one role"
      tone="success"
    >
      {#snippet icon()}<Users size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Coverage quality"
      value={`${permissionCoverage}%`}
      delta={`${data.roles.filter((role) => role.client_id).length} client roles`}
      meta="roles carrying explicit permission sets"
      tone="primary"
    >
      {#snippet icon()}<Shield size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <section class="roles-page__content">
    <SectionCard
      eyebrow="Role directory"
      title="Searchable roles"
      description="Each entry carries scope, intent, and assignee context."
    >
      <div class="roles-page__list">
        {#if visibleRoles.length > 0}
          {#each visibleRoles as role (role.id)}
            <div class="roles-page__row">
              <div>
                <strong>{role.name}</strong>
                <small>{roleDescription(role)}</small>
              </div>
              <div>
                <strong>{roleScope(role)}</strong>
                <small>{roleAssignments(role)}</small>
              </div>
            </div>
          {/each}
        {:else}
          <div class="roles-page__empty">
            <strong>No roles in this segment.</strong>
            <small>Switch the active role view to inspect a different scope.</small>
          </div>
        {/if}
      </div>
    </SectionCard>

    <div class="roles-page__stack">
      <SectionCard eyebrow="Permissions" title="Enabled groups" compact={true}>
        <div class="roles-page__meters">
          <LinearMeter
            label="Identity management"
            value={permissionCoverage}
            meta="roles with mapped permission payloads"
          />
          <LinearMeter
            label="Security policies"
            value={Math.min(100, Math.round((assignedUsers / Math.max(data.users.length, 1)) * 100))}
            meta="users covered by at least one role"
            tone="var(--success)"
          />
          <LinearMeter
            label="Realm configuration"
            value={data.roles.length === 0 ? 0 : Math.min(100, realmRoles * 10)}
            meta="breadth of realm-scoped definitions"
            tone="var(--warning)"
          />
        </div>
      </SectionCard>

      <SectionCard eyebrow="Selected role" title={selectedRole?.name ?? 'No role selected'} compact={true}>
        {#if selectedRole}
          <div class="roles-page__detail-grid">
            <div><span>Scope</span><strong>{roleScope(selectedRole)}</strong></div>
            <div><span>Permissions</span><strong>{selectedRole.permissions.length} enabled</strong></div>
            <div><span>Users</span><strong>{roleAssignments(selectedRole)}</strong></div>
            <div><span>Client link</span><strong>{selectedRole.client_id ? 'Attached' : 'None'}</strong></div>
          </div>
        {:else}
          <div class="roles-page__empty roles-page__empty--compact">
            <strong>No role loaded.</strong>
            <small>Create or import a role to inspect details here.</small>
          </div>
        {/if}
      </SectionCard>
    </div>
  </section>
</div>

<style>
  .roles-page,
  .roles-page__metrics,
  .roles-page__content,
  .roles-page__stack,
  .roles-page__list,
  .roles-page__meters,
  .roles-page__detail-grid,
  .roles-page__empty {
    display: grid;
    gap: 24px;
  }

  .roles-page__hero {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 20px;
    padding: 24px;
  }

  h2 {
    margin: 8px 0 10px;
    font: 700 clamp(2rem, 4vw, 3.2rem)/0.98 var(--font-display);
    letter-spacing: -0.05em;
  }

  p,
  span,
  small {
    margin: 0;
    color: var(--text-muted);
  }

  .roles-page__metrics {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .roles-page__content {
    grid-template-columns: minmax(0, 1.1fr) minmax(320px, 0.9fr);
  }

  .roles-page__row,
  .roles-page__detail-grid div {
    display: grid;
    gap: 4px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .roles-page__row {
    grid-template-columns: minmax(0, 1fr) 160px;
    align-items: center;
  }

  strong {
    color: var(--text);
  }

  .roles-page__empty {
    gap: 8px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px dashed var(--border);
  }

  .roles-page__empty--compact {
    gap: 6px;
    padding: 0;
    background: transparent;
    border: 0;
  }

  .roles-page__detail-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  @media (max-width: 1000px) {
    .roles-page__metrics,
    .roles-page__content,
    .roles-page__row,
    .roles-page__detail-grid {
      grid-template-columns: 1fr;
    }

    .roles-page__hero {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
