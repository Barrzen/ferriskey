<script lang="ts">
  import { ArrowRight, Sparkles } from 'lucide-svelte';
  import { ripple } from '$lib/utils/ripple';

  let {
    realm,
    title,
    description,
    highlights
  }: {
    realm: string;
    title: string;
    description: string;
    highlights: string[];
  } = $props();
</script>

<section class="placeholder glass-panel">
  <div class="placeholder__hero">
    <div class="placeholder__eyebrow">
      <Sparkles size={16} />
      <span>In active migration</span>
    </div>
    <h1>{title}</h1>
    <p>{description}</p>
    <a href={`/realms/${realm}/overview`} class="placeholder__link" use:ripple>
      Back to overview
      <ArrowRight size={16} />
    </a>
  </div>

  <div class="placeholder__grid">
    {#each highlights as item}
      <article>
        <span class="status-dot"></span>
        <p>{item}</p>
      </article>
    {/each}
  </div>
</section>

<style>
  .placeholder {
    display: grid;
    gap: 28px;
    padding: clamp(24px, 4vw, 40px);
  }

  .placeholder__hero {
    max-width: 720px;
  }

  .placeholder__eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 999px;
    background: var(--primary-soft);
    color: var(--primary);
    font-weight: 700;
    font-size: 0.9rem;
  }

  h1 {
    margin: 18px 0 10px;
    font: 700 clamp(2rem, 5vw, 3.4rem)/0.96 var(--font-display);
    letter-spacing: -0.05em;
  }

  p {
    margin: 0;
    color: var(--text-soft);
    line-height: 1.7;
  }

  .placeholder__link {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    margin-top: 24px;
    padding: 14px 18px;
    border-radius: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-md);
    font-weight: 700;
  }

  .placeholder__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 18px;
  }

  article {
    display: flex;
    gap: 14px;
    align-items: flex-start;
    padding: 18px;
    border-radius: 18px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
  }
</style>
