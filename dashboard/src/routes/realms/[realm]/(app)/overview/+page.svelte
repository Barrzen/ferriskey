<script lang="ts">
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

  const currentRealm = String(page.params.realm ?? 'master');
  const weeklyTraffic = [
    18, 26, 24, 31, 35, 29, 38, 42, 39, 46, 51, 49, 54, 58
  ];
  const downloadBars = [76, 58, 91, 66, 84, 72, 95];
  const sourceRows = [
    { label: 'Direct sign-in', value: '38%', tone: 'var(--primary)' },
    { label: 'Social login', value: '26%', tone: 'var(--success)' },
    { label: 'Magic links', value: '18%', tone: 'var(--warning)' },
    { label: 'Service accounts', value: '18%', tone: 'var(--text-muted)' }
  ];
  const moduleRows = [
    {
      name: 'Users workspace',
      progress: '92%',
      health: 'Stable',
      tone: 'var(--success)'
    },
    {
      name: 'Clients and scopes',
      progress: '74%',
      health: 'In progress',
      tone: 'var(--primary)'
    },
    {
      name: 'Security analytics',
      progress: '61%',
      health: 'Reviewing',
      tone: 'var(--warning)'
    },
    {
      name: 'Provider federation',
      progress: '48%',
      health: 'Queued',
      tone: 'var(--text-muted)'
    }
  ];
  const activityRows = [
    {
      title: 'Bulk invitation campaign completed',
      meta: '2 minutes ago',
      tone: 'var(--success)'
    },
    {
      title: 'Client secret rotation requested',
      meta: '16 minutes ago',
      tone: 'var(--primary)'
    },
    {
      title: 'Blocked 4 password spray attempts',
      meta: '28 minutes ago',
      tone: 'var(--danger)'
    },
    {
      title: 'New LDAP sync dry run finished',
      meta: '51 minutes ago',
      tone: 'var(--warning)'
    }
  ];
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
      <p>Weekly sales</p>
      <strong>$31,480</strong>
      <span>+18.4% compared with last week</span>
      <div class="overview__hero-kpis">
        <div>
          <small>Auth success</small>
          <strong>92.7%</strong>
        </div>
        <div>
          <small>MFA adoption</small>
          <strong>84.1%</strong>
        </div>
        <div>
          <small>Incidents</small>
          <strong>04</strong>
        </div>
      </div>
    </div>
  </section>

  <section class="overview__metrics">
    <MetricCard
      title="Active identities"
      value="18,426"
      delta="+8.4%"
      meta="compared with last week"
    >
      {#snippet icon()}<UserRoundCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Successful sign-ins"
      value="92.7%"
      delta="+1.6%"
      meta="security posture improving"
      tone="success"
    >
      {#snippet icon()}<ShieldCheck size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Threat events blocked"
      value="148"
      delta="+12.1%"
      meta="password spray and token abuse"
      tone="warning"
    >
      {#snippet icon()}<Activity size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
    <MetricCard
      title="Flow revisions"
      value="29"
      delta="+4.0%"
      meta="staged in Compass"
      tone="primary"
    >
      {#snippet icon()}<Waypoints size={24} strokeWidth={2.2} />{/snippet}
    </MetricCard>
  </section>

  <section class="overview__content">
    <article class="overview__card glass-panel">
      <div class="overview__section-head">
        <div>
          <p>Website visits</p>
          <h3>Authentication traffic keeps rising with controlled risk.</h3>
        </div>
        <strong>58k</strong>
      </div>

      <TrendAreaChart points={weeklyTraffic} />

      <div class="overview__legend">
        <div><span style="color: var(--primary)"></span>Interactive users</div>
        <div><span style="color: var(--success)"></span>SSO completions</div>
        <div><span style="color: var(--warning)"></span>Bot mitigation</div>
      </div>
    </article>

    <div class="overview__stack">
      <article class="overview__card glass-panel">
        <div class="overview__section-head overview__section-head--compact">
          <div>
            <p>Current visits</p>
            <h3>Sign-in mix</h3>
          </div>
        </div>

        <div class="overview__gauges">
          <RingGauge
            value={57}
            label="Desktop"
            meta="still the primary admin surface"
          />
          <RingGauge
            value={31}
            label="Mobile"
            meta="realm admins on the move"
            color="var(--success)"
          />
          <RingGauge
            value={12}
            label="API"
            meta="service accounts and jobs"
            color="var(--warning)"
          />
        </div>
      </article>

      <article class="overview__card glass-panel">
        <div class="overview__section-head overview__section-head--compact">
          <div>
            <p>Traffic by source</p>
            <h3>Current download</h3>
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
          <p>Migration health</p>
          <h3>Core dashboard modules moving into the new design system.</h3>
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
          <p>Order timeline</p>
          <h3>Latest realm and security activity.</h3>
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
