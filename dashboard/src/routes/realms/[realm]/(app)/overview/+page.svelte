<script lang="ts">
  import type { PageData } from './$types';
  import {
    Activity,
    ArrowUpRight,
    BadgeCheck,
    ShieldCheck,
    UserRoundCheck,
    Waypoints
  } from 'lucide-svelte';
  import { page } from '$app/state';
  import MetricCard from '$components/MetricCard.svelte';
  import RingGauge from '$components/RingGauge.svelte';
  import TrendAreaChart from '$components/TrendAreaChart.svelte';
  import { ripple } from '$utils/ripple';

  let { data }: { data: PageData } = $props();
  const currentRealm = String(page.params.realm ?? 'master');

  const activeUsers = $derived(data.users.filter((user) => user.enabled).length);
  const serviceAccounts = $derived(
    data.users.filter((user) => Boolean(user.client_id)).length
  );
  const verifiedUsers = $derived(
    data.users.filter((user) => user.email_verified).length
  );
  const verifiedRate = $derived(
    data.users.length === 0
      ? 0
      : Math.round((verifiedUsers / data.users.length) * 1000) / 10
  );
  const pendingActions = $derived(
    data.users.filter((user) => user.required_actions.length > 0).length
  );
  const confidentialClients = $derived(
    data.clients.filter((client) => !client.public_client).length
  );
  const publicClients = $derived(
    data.clients.filter((client) => client.public_client).length
  );
  const serviceClients = $derived(
    data.clients.filter((client) => client.service_account_enabled).length
  );
  const directGrantClients = $derived(
    data.clients.filter((client) => client.direct_access_grants_enabled).length
  );
  const secretCoverage = $derived(
    data.clients.length === 0
      ? 0
      : Math.round(
          (data.clients.filter((client) => !client.public_client && client.secret).length /
            data.clients.length) *
            1000
        ) / 10
  );
  const averagePermissions = $derived(
    data.roles.length === 0
      ? 0
      : Math.round(
          data.roles.reduce((total, role) => total + role.permissions.length, 0) /
            data.roles.length
        )
  );
  const weeklyTraffic = $derived([
    Math.max(12, data.users.length),
    Math.max(18, activeUsers),
    Math.max(16, verifiedUsers),
    Math.max(20, activeUsers + publicClients),
    Math.max(24, activeUsers + confidentialClients),
    Math.max(26, activeUsers + data.roles.length),
    Math.max(28, activeUsers + data.clients.length)
  ]);
  const downloadBars = $derived([
    Math.max(24, confidentialClients),
    Math.max(20, publicClients),
    Math.max(18, serviceClients),
    Math.max(14, pendingActions),
    Math.max(22, data.roles.length),
    Math.max(20, directGrantClients),
    Math.max(18, Math.round(secretCoverage))
  ]);
  const sourceRows = $derived([
    {
      label: 'Members',
      value: `${data.users.length === 0 ? 0 : Math.round(((data.users.length - serviceAccounts) / data.users.length) * 100)}%`,
      tone: 'var(--primary)'
    },
    {
      label: 'Service accounts',
      value: `${data.users.length === 0 ? 0 : Math.round((serviceAccounts / data.users.length) * 100)}%`,
      tone: 'var(--success)'
    },
    {
      label: 'Public clients',
      value: `${data.clients.length === 0 ? 0 : Math.round((publicClients / data.clients.length) * 100)}%`,
      tone: 'var(--warning)'
    },
    {
      label: 'Confidential clients',
      value: `${data.clients.length === 0 ? 0 : Math.round((confidentialClients / data.clients.length) * 100)}%`,
      tone: 'var(--text-muted)'
    }
  ]);
  const moduleRows = $derived([
    {
      name: 'Users workspace',
      progress: `${Math.min(100, 40 + data.users.length)}%`,
      health: pendingActions > 0 ? 'Needs follow-up' : 'Stable',
      tone: pendingActions > 0 ? 'var(--warning)' : 'var(--success)'
    },
    {
      name: 'Clients and scopes',
      progress: `${Math.min(100, 38 + data.clients.length)}%`,
      health: secretCoverage >= 60 ? 'Stable' : 'Reviewing',
      tone: secretCoverage >= 60 ? 'var(--primary)' : 'var(--warning)'
    },
    {
      name: 'Role model',
      progress: `${Math.min(100, 34 + data.roles.length)}%`,
      health: averagePermissions > 0 ? 'Mapped' : 'Empty',
      tone: averagePermissions > 0 ? 'var(--success)' : 'var(--text-muted)'
    },
    {
      name: 'Auth journeys',
      progress: `${pendingActions === 0 ? 88 : 64}%`,
      health: pendingActions === 0 ? 'Ready' : 'In progress',
      tone: pendingActions === 0 ? 'var(--success)' : 'var(--primary)'
    }
  ]);
  const activityRows = $derived([
    {
      title: `${activeUsers} active identities available in ${currentRealm}`,
      meta: `${verifiedUsers} verified emails and ${pendingActions} pending actions`,
      tone: 'var(--success)'
    },
    {
      title: `${data.clients.length} clients currently registered`,
      meta: `${confidentialClients} confidential and ${publicClients} public clients`,
      tone: 'var(--primary)'
    },
    {
      title: `${data.roles.length} roles define access boundaries`,
      meta: `${averagePermissions} permissions per role on average`,
      tone: 'var(--warning)'
    },
    {
      title: `${serviceAccounts} service accounts support automation`,
      meta: `${serviceClients} clients expose service account flows`,
      tone: 'var(--text-muted)'
    }
  ]);
</script>

<div class="overview grid-auto">
  <section class="overview__hero glass-panel">
    <div class="overview__hero-copy">
      <div class="overview__eyebrow">
        <span class="status-dot"></span>
        Barrzen Minimal Design
      </div>
      <h2>Operate identity at a glance, without losing depth.</h2>
      <p>
        This overview now follows the Barrzen Minimal dashboard direction:
        quieter surfaces, sharper hierarchy, cleaner density, and clearer
        pathways into realm operations.
      </p>

      <div class="overview__hero-actions">
        <a
          href={`/realms/${currentRealm}/users`}
          class="overview__cta overview__cta--primary"
          use:ripple
        >
          Open users
          <ArrowUpRight size={16} />
        </a>
        <a
          href={`/realms/${currentRealm}/authentication/login`}
          class="overview__cta"
          use:ripple
        >
          Preview login
        </a>
      </div>
    </div>

    <div class="overview__hero-panel">
      <p>Realm footprint</p>
      <strong>{activeUsers}</strong>
      <span>{verifiedRate}% of identities have verified email</span>
      <div class="overview__hero-kpis">
        <div>
          <small>Clients</small>
          <strong>{data.clients.length}</strong>
        </div>
        <div>
          <small>Roles</small>
          <strong>{data.roles.length}</strong>
        </div>
        <div>
          <small>Pending</small>
          <strong>{pendingActions}</strong>
        </div>
      </div>
    </div>
  </section>

  <section class="overview__metrics">
    <MetricCard
      title="Active identities"
      value={String(activeUsers)}
      delta={`${serviceAccounts} services`}
      meta="live realm members and automations"
    >
      {#snippet icon()}<UserRoundCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Verified identities"
      value={`${verifiedRate}%`}
      delta={`${verifiedUsers} accounts`}
      meta="email verification completion"
      tone="success"
    >
      {#snippet icon()}<ShieldCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Clients in scope"
      value={String(data.clients.length)}
      delta={`${confidentialClients} confidential`}
      meta="registered applications and services"
      tone="warning"
    >
      {#snippet icon()}<Activity size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Role definitions"
      value={String(data.roles.length)}
      delta={`${averagePermissions} perms avg`}
      meta="permission sets across realm access"
      tone="primary"
    >
      {#snippet icon()}<Waypoints size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <section class="overview__content">
    <article class="overview__card glass-panel">
        <div class="overview__section-head">
          <div>
            <p>Identity growth</p>
            <h3>Live realm counts now shape the overview instead of placeholder metrics.</h3>
          </div>
          <strong>{activeUsers + data.clients.length + data.roles.length}</strong>
        </div>

      <TrendAreaChart points={weeklyTraffic} />

        <div class="overview__legend">
          <div><span style="color: var(--primary)"></span>Active identities</div>
          <div><span style="color: var(--success)"></span>Verified accounts</div>
          <div><span style="color: var(--warning)"></span>Realm resources</div>
        </div>
    </article>

    <div class="overview__stack">
      <article class="overview__card glass-panel">
        <div class="overview__section-head overview__section-head--compact">
          <div>
            <p>Directory mix</p>
            <h3>Identity composition</h3>
          </div>
        </div>

        <div class="overview__gauges">
          <RingGauge
            value={data.users.length === 0 ? 0 : Math.round(((data.users.length - serviceAccounts) / data.users.length) * 100)}
            label="Members"
            meta="human identities in the realm"
          />
          <RingGauge
            value={data.users.length === 0 ? 0 : Math.round((serviceAccounts / data.users.length) * 100)}
            label="Services"
            meta="automation and machine access"
            color="var(--success)"
          />
          <RingGauge
            value={verifiedRate}
            label="Verified"
            meta="email verification posture"
            color="var(--warning)"
          />
        </div>
      </article>

      <article class="overview__card glass-panel">
        <div class="overview__section-head overview__section-head--compact">
          <div>
            <p>Realm distribution</p>
            <h3>Current split</h3>
          </div>
        </div>

        <div class="overview__sources">
          {#each sourceRows as row (row.label)}
            <div>
              <div>
                <span class="overview__source-dot" style={`color:${row.tone}`}
                ></span>
                <strong>{row.label}</strong>
              </div>
              <small>{row.value}</small>
            </div>
          {/each}
        </div>

        <div class="overview__bars">
          {#each downloadBars as bar, index (`${bar}-${index}`)}
            <span style={`height:${bar}%`}></span>
          {/each}
        </div>
      </article>
    </div>
  </section>

  <section class="overview__content overview__content--bottom">
    <article class="overview__card glass-panel">
        <div class="overview__section-head">
          <div>
            <p>Live module posture</p>
            <h3>Realm data now drives the key surfaces already migrated into the new UI.</h3>
          </div>
          <BadgeCheck size={20} color="var(--primary)" />
        </div>

      <div class="overview__table">
        {#each moduleRows as row (row.name)}
          <div class="overview__table-row">
            <div>
              <strong>{row.name}</strong>
              <span>{row.health}</span>
            </div>
            <div class="overview__progress">
              <small>{row.progress}</small>
              <div>
                <span style={`width:${row.progress}; background:${row.tone}`}
                ></span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </article>

    <article class="overview__card glass-panel">
        <div class="overview__section-head">
          <div>
            <p>Realm snapshot</p>
            <h3>Current operational signals from live users, clients, and roles.</h3>
          </div>
        </div>

      <div class="overview__timeline">
        {#each activityRows as item (item.title)}
          <section>
            <span class="status-dot" style={`color:${item.tone}`}></span>
            <div>
              <strong>{item.title}</strong>
              <p>{item.meta}</p>
            </div>
          </section>
        {/each}
      </div>
    </article>
  </section>
</div>

<style>
  .overview__hero,
  .overview__card {
    padding: 24px;
  }

  .overview__hero {
    display: grid;
    grid-template-columns: minmax(0, 1.35fr) minmax(300px, 0.7fr);
    gap: 24px;
    align-items: stretch;
  }

  .overview__hero-copy,
  .overview__hero-panel,
  .overview__stack {
    display: grid;
    gap: 16px;
  }

  .overview__eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 9px 14px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.86rem;
    font-weight: 700;
    justify-self: start;
  }

  h2 {
    margin: 0;
    font: 700 clamp(2.1rem, 5vw, 3.6rem)/0.96 var(--font-display);
    letter-spacing: -0.06em;
  }

  p {
    margin: 0;
    color: var(--text-soft);
    line-height: 1.7;
  }

  .overview__hero-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .overview__cta {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 14px 18px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    font-weight: 700;
  }

  .overview__cta--primary {
    background: var(--primary);
    border-color: transparent;
    color: white;
  }

  .overview__hero-panel {
    padding: 20px;
    border-radius: 20px;
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--primary) 12%, var(--surface)) 0%,
      var(--surface) 100%
    );
    border: 1px solid var(--border);
  }

  .overview__hero-panel p,
  .overview__section-head p {
    font-size: 0.86rem;
  }

  .overview__hero-panel strong,
  .overview__section-head strong,
  h3 {
    font: 700 1.45rem/1.06 var(--font-display);
    letter-spacing: -0.04em;
  }

  .overview__hero-panel > strong {
    font-size: 2.4rem;
  }

  .overview__hero-panel > span {
    color: var(--success);
    font-size: 0.92rem;
    font-weight: 700;
  }

  .overview__hero-kpis {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
  }

  .overview__hero-kpis div {
    padding: 14px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .overview__hero-kpis small,
  .overview__table-row span,
  .overview__progress small,
  .overview__sources small {
    color: var(--text-muted);
  }

  .overview__metrics {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 24px;
  }

  .overview__content {
    display: grid;
    grid-template-columns: minmax(0, 1.4fr) minmax(320px, 0.86fr);
    gap: 24px;
  }

  .overview__content--bottom {
    grid-template-columns: minmax(0, 1.05fr) minmax(320px, 0.95fr);
  }

  .overview__section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;
  }

  .overview__section-head--compact {
    margin-bottom: 16px;
  }

  .overview__section-head strong {
    font-size: 1.2rem;
  }

  .overview__legend,
  .overview__sources,
  .overview__gauges,
  .overview__timeline,
  .overview__table {
    display: grid;
    gap: 16px;
  }

  .overview__legend {
    grid-template-columns: repeat(3, minmax(0, 1fr));
    font-size: 0.9rem;
    color: var(--text-soft);
  }

  .overview__legend div,
  .overview__sources > div {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
  }

  .overview__legend span,
  .overview__source-dot {
    display: inline-flex;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: currentColor;
    box-shadow: 0 0 0 5px color-mix(in srgb, currentColor 18%, transparent);
    margin-right: 8px;
  }

  .overview__sources > div {
    padding: 14px 0;
    border-bottom: 1px dashed var(--border);
  }

  .overview__sources > div:last-child {
    padding-bottom: 0;
    border-bottom: 0;
  }

  .overview__sources > div > div {
    display: flex;
    align-items: center;
  }

  .overview__bars {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    align-items: end;
    gap: 8px;
    height: 120px;
    padding-top: 8px;
  }

  .overview__bars span {
    border-radius: 999px 999px 10px 10px;
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--primary) 62%, white),
      var(--primary)
    );
  }

  .overview__table-row,
  .overview__timeline section {
    display: grid;
    gap: 12px;
    padding: 16px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .overview__table-row {
    grid-template-columns: minmax(0, 1fr) 180px;
    align-items: center;
  }

  .overview__table-row strong,
  .overview__timeline strong {
    display: block;
    margin-bottom: 4px;
  }

  .overview__progress {
    display: grid;
    gap: 8px;
  }

  .overview__progress div {
    height: 8px;
    border-radius: 999px;
    background: color-mix(in srgb, var(--text-muted) 16%, transparent);
    overflow: hidden;
  }

  .overview__progress span {
    display: block;
    height: 100%;
    border-radius: inherit;
  }

  .overview__timeline section {
    grid-template-columns: auto 1fr;
    align-items: start;
  }

  @media (max-width: 1200px) {
    .overview__hero,
    .overview__metrics,
    .overview__content,
    .overview__content--bottom,
    .overview__hero-kpis {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 900px) {
    .overview__hero,
    .overview__metrics,
    .overview__content,
    .overview__content--bottom,
    .overview__hero-kpis,
    .overview__gauges,
    .overview__legend,
    .overview__table-row {
      grid-template-columns: 1fr;
    }

    .overview__hero,
    .overview__card {
      padding: 18px;
    }
  }
</style>
