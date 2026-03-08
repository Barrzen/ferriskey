<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    title,
    value,
    delta,
    meta,
    tone = 'primary',
    icon
  }: {
    title: string;
    value: string;
    delta: string;
    meta: string;
    tone?: 'primary' | 'success' | 'warning' | 'danger';
    icon: Snippet;
  } = $props();
</script>

<section class="metric-card glass-panel">
  <div class={`metric-card__icon metric-card__icon--${tone}`}>
    {@render icon()}
  </div>
  <div>
    <p class="metric-card__title">{title}</p>
    <h3>{value}</h3>
    <div class="metric-card__meta">
      <span>{delta}</span>
      <span>{meta}</span>
    </div>
  </div>
</section>

<style>
  .metric-card {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 18px;
    padding: 22px;
  }

  .metric-card__icon {
    display: grid;
    place-items: center;
    width: 56px;
    height: 56px;
    border-radius: 20px;
  }

  .metric-card__icon--primary {
    background: color-mix(in srgb, var(--primary) 18%, transparent);
    color: var(--primary);
  }

  .metric-card__icon--success {
    background: color-mix(in srgb, var(--success) 18%, transparent);
    color: var(--success);
  }

  .metric-card__icon--warning {
    background: color-mix(in srgb, var(--warning) 20%, transparent);
    color: var(--warning);
  }

  .metric-card__icon--danger {
    background: color-mix(in srgb, var(--danger) 18%, transparent);
    color: var(--danger);
  }

  .metric-card__title,
  .metric-card__meta {
    margin: 0;
  }

  h3 {
    margin: 6px 0;
    font: 700 1.7rem/1 var(--font-display);
    letter-spacing: -0.04em;
  }

  .metric-card__title {
    color: var(--text-muted);
    font-size: 0.92rem;
  }

  .metric-card__meta {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    font-size: 0.88rem;
    color: var(--text-soft);
  }

  .metric-card__meta span:first-child {
    color: var(--success);
    font-weight: 700;
  }
</style>
