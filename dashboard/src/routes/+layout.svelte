<script lang="ts">
  import { QueryClientProvider } from '@tanstack/svelte-query';
  import '../app.css';
  import { queryClient } from '$lib/query/client';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href="/fav.png" type="image/png" />
  <title>Barrzen Minimal Dashboard</title>
  <meta
    name="description"
    content="Barrzen Minimal Dashboard built in SvelteKit for the next-generation IAM admin experience."
  />
</svelte:head>

<QueryClientProvider client={queryClient}>
  {@render children()}
</QueryClientProvider>
