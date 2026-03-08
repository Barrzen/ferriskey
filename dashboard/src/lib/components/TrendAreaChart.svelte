<script lang="ts">
  let { points, tone = 'var(--primary)' }: { points: number[]; tone?: string } =
    $props();

  const width = 480;
  const height = 220;
  const padding = 12;

  const chart = $derived.by(() => {
    const min = Math.min(...points);
    const max = Math.max(...points);
    const range = Math.max(max - min, 1);

    const mapped = points.map((point, index) => {
      const x = (index / (points.length - 1)) * (width - padding * 2) + padding;
      const y =
        height - padding - ((point - min) / range) * (height - padding * 2);
      return [x, y] as const;
    });

    const line = mapped.map(([x, y]) => `${x},${y}`).join(' ');
    const area = `${padding},${height - padding} ${line} ${width - padding},${height - padding}`;

    return { area, line };
  });
</script>

<svg
  viewBox={`0 0 ${width} ${height}`}
  class="trend-chart"
  preserveAspectRatio="none"
  aria-hidden="true"
>
  <defs>
    <linearGradient id="trendFill" x1="0" x2="0" y1="0" y2="1">
      <stop offset="0%" stop-color={tone} stop-opacity="0.26" />
      <stop offset="100%" stop-color={tone} stop-opacity="0.02" />
    </linearGradient>
  </defs>
  <g>
    {#each [0, 1, 2, 3] as tick}
      <line
        x1={padding}
        x2={width - padding}
        y1={padding + tick * ((height - padding * 2) / 3)}
        y2={padding + tick * ((height - padding * 2) / 3)}
      />
    {/each}
  </g>
  <polygon points={chart.area} fill="url(#trendFill)" />
  <polyline
    points={chart.line}
    stroke={tone}
    stroke-width="4"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
</svg>

<style>
  .trend-chart {
    width: 100%;
    height: 220px;
  }

  line {
    stroke: color-mix(in srgb, var(--text-muted) 18%, transparent);
    stroke-width: 1;
  }
</style>
