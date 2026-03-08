<script lang="ts">
  let {
    size = 'regular',
    showLabel = true
  }: { size?: 'compact' | 'regular'; showLabel?: boolean } = $props();
</script>

<div class:brand-logo--compact={size === 'compact'} class="brand-logo">
  <img
    src="/brand/barrzen.png"
    alt="Barrzen"
    width="420"
    height="88"
    class="brand-logo__image"
  />
  {#if showLabel}
    <div class="brand-logo__meta">
      <strong>Barrzen</strong>
      <span>Minimal Design dashboard</span>
    </div>
  {/if}
</div>

<style>
  .brand-logo {
    display: inline-flex;
    align-items: center;
    gap: 14px;
  }

  .brand-logo__image {
    height: 24px;
    width: auto;
    object-fit: contain;
  }

  .brand-logo__meta {
    display: grid;
    gap: 2px;
  }

  .brand-logo__meta strong {
    font-size: 0.98rem;
    line-height: 1;
  }

  .brand-logo__meta span {
    color: var(--text-muted);
    font-size: 0.78rem;
    line-height: 1;
  }

  .brand-logo--compact .brand-logo__image {
    height: 20px;
  }
</style>
