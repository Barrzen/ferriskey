<script lang="ts">
  import { page } from '$app/state';
  import {
    Activity,
    ArrowUpRight,
    ShieldCheck,
    UserRoundCheck,
    Waypoints
  } from 'lucide-svelte';
  import MetricCard from '$components/MetricCard.svelte';
  import RingGauge from '$components/RingGauge.svelte';
  import TrendAreaChart from '$components/TrendAreaChart.svelte';
  import { ripple } from '$utils/ripple';

  const currentRealm = String(page.params.realm ?? 'master');
  const authTraffic = [18, 24, 21, 29, 32, 26, 34, 39, 36, 44, 48, 46, 52, 54];

  const queueRows = [
    {
      name: 'Realm login hardening',
      owner: 'Security',
      progress: '92%',
      status: 'Ready for rollout'
    },
    {
      name: 'User import from HRIS',
      owner: 'Automation',
      progress: '68%',
      status: 'Sync in progress'
    },
    {
      name: 'Client secret rotation',
      owner: 'Platform',
      progress: '51%',
      status: 'Waiting approval'
    },
    {
      name: 'TOTP adoption campaign',
      owner: 'IAM',
      progress: '34%',
      status: 'Drafting notices'
    }
  ];

  const events = [
    {
      tone: 'var(--success)',
      title: '2,341 passwordless sign-ins',
      meta: 'Healthy conversion in the last 24 hours'
    },
    {
      tone: 'var(--warning)',
      title: '12 stale service accounts',
      meta: 'Rotation due within the next 3 days'
    },
    {
      tone: 'var(--danger)',
      title: '4 brute-force spikes blocked',
      meta: 'All mitigated by realm policy'
    }
  ];
</script>

<div class="overview grid-auto">
  <section class="overview__hero glass-panel">
    <div>
      <div class="overview__eyebrow">
        <span class="status-dot"></span>
        Realm posture is healthy
      </div>
      <h2>Minimal polish for identity operations.</h2>
      <p>
        This new dashboard is built as a first-class SvelteKit app, keeping
        realm-aware workflows while improving clarity, depth, motion, and dark
        mode behavior.
      </p>
    </div>

    <div class="overview__actions">
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
  </section>

  <section class="overview__metrics">
    <MetricCard
      title="Active identities"
      value="18,426"
      delta="+8.4%"
      meta="from the last 7 days"
    >
      {#snippet icon()}<UserRoundCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Successful sign-ins"
      value="92.7%"
      delta="+1.6%"
      meta="improved this week"
      tone="success"
    >
      {#snippet icon()}<ShieldCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Threat events blocked"
      value="148"
      delta="+12.1%"
      meta="rate-limits and lockouts"
      tone="warning"
    >
      {#snippet icon()}<Activity size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Flow revisions"
      value="29"
      delta="+4.0%"
      meta="Compass changes staged"
      tone="primary"
    >
      {#snippet icon()}<Waypoints size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <section class="overview__content">
    <article class="overview__chart glass-panel">
      <div class="overview__section-head">
        <div>
          <p>Authentication traffic</p>
          <h3>Requests are trending up, without increasing risk.</h3>
        </div>
        <strong>54k / hour</strong>
      </div>

      <TrendAreaChart points={authTraffic} />

      <div class="overview__legend">
        <div><span></span>Interactive logins</div>
        <div><span></span>SSO continuation</div>
        <div><span></span>Service accounts</div>
      </div>
    </article>

    <article class="overview__gauges glass-panel">
      <div class="overview__section-head">
        <div>
          <p>Realm reliability</p>
          <h3>Operational posture at a glance</h3>
        </div>
      </div>

      <div class="overview__gauge-grid">
        <RingGauge
          value={97}
          label="Availability"
          meta="No auth outage in the last 30 days"
        />
        <RingGauge
          value={84}
          label="MFA adoption"
          meta="Recovery codes issued for 71% of enrolled users"
          color="var(--success)"
        />
        <RingGauge
          value={63}
          label="Automation"
          meta="Client provisioning flows still moving to the new UI"
          color="var(--warning)"
        />
      </div>
    </article>
  </section>

  <section class="overview__tables">
    <article class="glass-panel overview__table-card">
      <div class="overview__section-head">
        <div>
          <p>Delivery queue</p>
          <h3>Migration work tracked in the dashboard itself</h3>
        </div>
      </div>

      <table>
        <thead>
          <tr>
            <th>Task</th>
            <th>Owner</th>
            <th>Progress</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {#each queueRows as row}
            <tr>
              <td>{row.name}</td>
              <td>{row.owner}</td>
              <td>{row.progress}</td>
              <td>{row.status}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </article>

    <article class="glass-panel overview__events-card">
      <div class="overview__section-head">
        <div>
          <p>SeaWatch signal</p>
          <h3>Priority events worth attention</h3>
        </div>
      </div>

      <div class="overview__events">
        {#each events as event}
          <section>
            <div class="status-dot" style={`color:${event.tone}`}></div>
            <div>
              <strong>{event.title}</strong>
              <p>{event.meta}</p>
            </div>
          </section>
        {/each}
      </div>
    </article>
  </section>
</div>

<style>
  .overview__hero,
  .overview__chart,
  .overview__gauges,
  .overview__table-card,
  .overview__events-card {
    padding: 26px;
  }

  .overview__hero {
    display: flex;
    justify-content: space-between;
    gap: 24px;
    align-items: flex-end;
    background:
      radial-gradient(
        circle at top right,
        color-mix(in srgb, var(--primary) 20%, transparent),
        transparent 28%
      ),
      linear-gradient(
        145deg,
        color-mix(in srgb, var(--surface) 92%, transparent),
        color-mix(in srgb, var(--surface-strong) 80%, transparent)
      );
  }

  .overview__eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-size: 0.92rem;
    font-weight: 700;
  }

  h2 {
    max-width: 620px;
    margin: 18px 0 12px;
    font: 700 clamp(2.2rem, 4vw, 3.7rem)/0.95 var(--font-display);
    letter-spacing: -0.06em;
  }

  p {
    margin: 0;
    color: var(--text-soft);
    line-height: 1.7;
  }

  .overview__actions {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
  }

  .overview__cta {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    padding: 15px 18px;
    border-radius: 18px;
    background: var(--surface);
    border: 1px solid var(--border);
    font-weight: 700;
    box-shadow: var(--shadow-md);
  }

  .overview__cta--primary {
    background: var(--primary);
    color: white;
    border-color: transparent;
  }

  .overview__metrics,
  .overview__content,
  .overview__tables {
    display: grid;
    gap: 24px;
  }

  .overview__metrics {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }

  .overview__content {
    grid-template-columns: minmax(0, 1.5fr) minmax(320px, 0.9fr);
  }

  .overview__tables {
    grid-template-columns: minmax(0, 1.2fr) minmax(320px, 0.8fr);
  }

  .overview__section-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 18px;
    margin-bottom: 24px;
  }

  .overview__section-head p {
    font-size: 0.9rem;
  }

  h3,
  .overview__section-head strong {
    margin: 4px 0 0;
    font: 700 1.35rem/1.1 var(--font-display);
    letter-spacing: -0.04em;
  }

  .overview__legend,
  .overview__gauge-grid,
  .overview__events {
    display: grid;
    gap: 16px;
  }

  .overview__legend {
    grid-template-columns: repeat(3, minmax(0, 1fr));
    font-size: 0.92rem;
    color: var(--text-soft);
  }

  .overview__legend div {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .overview__legend span {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--primary);
  }

  .overview__legend div:nth-child(2) span {
    background: var(--success);
  }

  .overview__legend div:nth-child(3) span {
    background: var(--warning);
  }

  .overview__gauge-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 14px 0;
    text-align: left;
    border-bottom: 1px solid var(--border);
  }

  th {
    color: var(--text-muted);
    font-size: 0.82rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  td {
    color: var(--text-soft);
  }

  td:first-child {
    color: var(--text);
    font-weight: 700;
  }

  .overview__events section {
    display: flex;
    gap: 14px;
    padding: 18px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }

  .overview__events strong {
    display: block;
    margin-bottom: 6px;
  }

  @media (max-width: 1200px) {
    .overview__metrics,
    .overview__content,
    .overview__tables,
    .overview__gauge-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 900px) {
    .overview__hero,
    .overview__metrics,
    .overview__content,
    .overview__tables,
    .overview__gauge-grid,
    .overview__legend {
      grid-template-columns: 1fr;
    }

    .overview__hero {
      align-items: flex-start;
      flex-direction: column;
    }

    table {
      display: block;
      overflow-x: auto;
    }
  }
</style>
