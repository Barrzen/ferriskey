<script lang="ts">
  let {
    value,
    label,
    meta,
    color = 'var(--primary)'
  }: {
    value: number;
    label: string;
    meta: string;
    color?: string;
  } = $props();
</script>

<section class="ring-gauge">
  <div class="ring-gauge__chart" style={`--progress:${value};--ring:${color};`}>
    <div>
      <strong>{value}%</strong>
      <span>{label}</span>
    </div>
  </div>
  <p>{meta}</p>
</section>

<style>
  .ring-gauge {
    display: grid;
    justify-items: center;
    gap: 14px;
  }

  .ring-gauge__chart {
    width: 138px;
    height: 138px;
    padding: 12px;
    border-radius: 50%;
    background:
      radial-gradient(circle at center, var(--surface) 58%, transparent 59%),
      conic-gradient(
        var(--ring) calc(var(--progress) * 1%),
        color-mix(in srgb, var(--ring) 16%, transparent) 0
      );
  }

  .ring-gauge__chart > div {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: var(--surface);
    border: 1px solid var(--border);
  }

  strong {
    font: 700 1.7rem/1 var(--font-display);
  }

  span,
  p {
    margin: 0;
    color: var(--text-soft);
    text-align: center;
  }
</style>
