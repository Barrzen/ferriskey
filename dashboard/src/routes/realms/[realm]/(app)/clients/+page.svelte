<script lang="ts">
  import type { PageData } from './$types';
  import { KeyRound, PanelsTopLeft, Rocket, Shield } from 'lucide-svelte';
  import ChipTabs from '$components/ChipTabs.svelte';
  import LinearMeter from '$components/LinearMeter.svelte';
  import MetricCard from '$components/MetricCard.svelte';
  import SectionCard from '$components/SectionCard.svelte';
  import { ripple } from '$utils/ripple';

  let { data }: { data: PageData } = $props();
  const tabs = ['All clients', 'Confidential', 'Public', 'Deprecated'];
  let activeTab = $state('All clients');

  function clientKind(client: PageData['clients'][number]) {
    if (client.service_account_enabled) {
      return 'Service account';
    }

    if (client.public_client) {
      return 'Public';
    }

    return 'Confidential';
  }

  function clientStatus(client: PageData['clients'][number]) {
    if (!client.enabled) {
      return 'Deprecated';
    }

    if (client.direct_access_grants_enabled) {
      return 'Review';
    }

    return 'Active';
  }

  const confidentialCount = $derived(
    data.clients.filter((client) => !client.public_client).length
  );
  const publicCount = $derived(
    data.clients.filter((client) => client.public_client).length
  );
  const deprecatedCount = $derived(
    data.clients.filter((client) => !client.enabled).length
  );
  const secretCount = $derived(
    data.clients.filter((client) => Boolean(client.secret)).length
  );
  const redirectCoverage = $derived(
    data.clients.length === 0
      ? 0
      : Math.round(
          (data.clients.filter(
            (client) => (client.redirect_uris?.length ?? 0) > 0 || client.service_account_enabled
          ).length /
            data.clients.length) *
            100
        )
  );
  const secretCoverage = $derived(
    data.clients.length === 0 ? 0 : Math.round((secretCount / data.clients.length) * 100)
  );
  const grantCoverage = $derived(
    data.clients.length === 0
      ? 0
      : Math.round(
          (data.clients.filter((client) => !client.direct_access_grants_enabled).length /
            data.clients.length) *
            100
        )
  );

  const visibleClients = $derived.by(() =>
    data.clients.filter((client) => {
      if (activeTab === 'Confidential') {
        return !client.public_client;
      }

      if (activeTab === 'Public') {
        return client.public_client;
      }

      if (activeTab === 'Deprecated') {
        return !client.enabled;
      }

      return true;
    })
  );

  const selectedClient = $derived(visibleClients[0] ?? data.clients[0] ?? null);
</script>

<div class="clients-page">
  <section class="clients-page__hero glass-panel">
    <div>
      <p>Clients workspace</p>
      <h2>Safer application setup with less buried configuration.</h2>
      <span
        >Credential visibility, redirect hygiene, and grant-type review now live
        in a more structured Barrzen Minimal layout.</span
      >
    </div>
    <div class="clients-page__hero-actions">
      <ChipTabs items={tabs} active={activeTab} tone="soft" onselect={(item) => (activeTab = item)} />
      <button type="button" class="clients-page__cta" use:ripple
        >New client</button
      >
    </div>
  </section>

  <section class="clients-page__metrics">
    <MetricCard
      title="Managed clients"
      value={String(data.clients.length)}
      delta={`${publicCount} public`}
      meta={`${confidentialCount} confidential clients`}
    >
      {#snippet icon()}<PanelsTopLeft size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Secret rotation"
      value={`${secretCoverage}%`}
      delta={`${secretCount} with secret`}
      meta="live credential coverage"
      tone="success"
    >
      {#snippet icon()}<KeyRound size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Grant hygiene"
      value={`${grantCoverage}%`}
      delta={`${deprecatedCount} deprecated`}
      meta="clients without direct grants"
      tone="primary"
    >
      {#snippet icon()}<Shield size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <section class="clients-page__content">
    <SectionCard
      eyebrow="Inventory"
      title="Registered clients"
      description="Searchable cards for IDs, types, states, and redirect footprint."
    >
      <div class="client-list">
        {#if visibleClients.length > 0}
          {#each visibleClients as client (client.id)}
            <div class="client-list__row">
              <div>
                <strong>{client.name}</strong>
                <small>{client.client_id}</small>
              </div>
              <div>
                <strong>{clientKind(client)}</strong>
                <small>
                  {#if client.service_account_enabled}
                    Service account enabled
                  {:else if (client.redirect_uris?.length ?? 0) > 0}
                    {client.redirect_uris?.length} redirect URI(s)
                  {:else}
                    No redirect URIs
                  {/if}
                </small>
              </div>
              <span>{clientStatus(client)}</span>
            </div>
          {/each}
        {:else}
          <div class="clients-page__empty">
            <strong>No clients in this slice.</strong>
            <small>Change the active tab to inspect a different segment.</small>
          </div>
        {/if}
      </div>
    </SectionCard>

    <div class="clients-page__stack">
      <SectionCard
        eyebrow="Current detail"
        title={selectedClient?.name ?? 'No client selected'}
        compact={true}
      >
        {#if selectedClient}
          <div class="clients-page__detail-grid">
            <div><span>Client type</span><strong>{clientKind(selectedClient)}</strong></div>
            <div><span>Protocol</span><strong>{selectedClient.protocol}</strong></div>
            <div>
              <span>Redirect URIs</span>
              <strong>
                {selectedClient.service_account_enabled
                  ? 'Service flow only'
                  : `${selectedClient.redirect_uris?.length ?? 0} managed`}
              </strong>
            </div>
            <div>
              <span>Secret status</span>
              <strong>{selectedClient.secret ? 'Configured' : 'Not exposed'}</strong>
            </div>
          </div>
        {:else}
          <div class="clients-page__empty clients-page__empty--compact">
            <strong>No client loaded.</strong>
            <small>This realm has no registered clients yet.</small>
          </div>
        {/if}
      </SectionCard>

      <SectionCard
        eyebrow="Capability config"
        title="Operational health"
        compact={true}
      >
        <div class="clients-page__meters">
          <LinearMeter
            label="Redirect URI hygiene"
            value={redirectCoverage}
            meta="clients expose redirect metadata or service flows"
          />
          <LinearMeter
            label="Secret freshness"
            value={secretCoverage}
            meta="confidential clients with visible secret material"
            tone="var(--success)"
          />
          <LinearMeter
            label="Grant simplification"
            value={grantCoverage}
            meta="direct access grants disabled where possible"
            tone="var(--warning)"
          />
        </div>
      </SectionCard>

      <SectionCard
        eyebrow="Credential panel"
        title="Current secret state"
        compact={true}
      >
        <div class="clients-page__credential-card">
          <strong>{selectedClient?.secret ? 'Secret available' : 'Secret not exposed'}</strong>
          <small>
            {#if selectedClient}
              {selectedClient.public_client
                ? 'This client is public and should not rely on a confidential secret.'
                : selectedClient.secret
                  ? 'The current API response includes secret material for this client.'
                  : 'The API does not currently expose a client secret for this entry.'}
            {:else}
              Select a client to inspect its credential posture.
            {/if}
          </small>
          <button type="button" use:ripple>Rotate secret</button>
        </div>
      </SectionCard>
    </div>
  </section>
</div>

<style>
  .clients-page,
  .clients-page__stack,
  .clients-page__metrics,
  .clients-page__content,
  .clients-page__meters,
  .client-list,
  .clients-page__detail-grid,
  .clients-page__empty {
    display: grid;
    gap: 24px;
  }

  .clients-page__hero {
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

  .clients-page__hero-actions {
    display: grid;
    gap: 12px;
    justify-items: end;
  }

  .clients-page__metrics {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .clients-page__content {
    grid-template-columns: minmax(0, 1.25fr) minmax(320px, 0.75fr);
  }

  .clients-page__cta,
  .clients-page__credential-card button {
    padding: 14px 16px;
    border-radius: 16px;
    border: 0;
    background: var(--primary);
    color: white;
    font-weight: 700;
    cursor: pointer;
  }

  .client-list__row,
  .clients-page__detail-grid div,
  .clients-page__credential-card {
    display: grid;
    gap: 4px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .client-list__row {
    grid-template-columns: minmax(0, 1fr) minmax(160px, 0.7fr) auto;
    align-items: center;
    gap: 16px;
  }

  .client-list__row span {
    padding: 8px 12px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.82rem;
    font-weight: 700;
  }

  strong {
    color: var(--text);
  }

  .clients-page__empty {
    gap: 8px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px dashed var(--border);
  }

  .clients-page__empty--compact {
    gap: 6px;
    padding: 0;
    background: transparent;
    border: 0;
  }

  .clients-page__detail-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .clients-page__credential-card button {
    justify-self: start;
    margin-top: 8px;
  }

  @media (max-width: 1000px) {
    .clients-page__metrics,
    .clients-page__content,
    .client-list__row,
    .clients-page__detail-grid {
      grid-template-columns: 1fr;
    }

    .clients-page__hero {
      flex-direction: column;
      align-items: flex-start;
    }

    .clients-page__hero-actions {
      justify-items: start;
    }
  }
</style>
