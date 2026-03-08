<script lang="ts">
  import { QueryClientProvider } from '@tanstack/svelte-query';
  import '../app.css';
  import favicon from '$lib/assets/favicon.svg';
  import { queryClient } from '$lib/query/client';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
  <title>FerrisKey Dashboard</title>
  <meta
    name="description"
    content="A Minimal-inspired SvelteKit dashboard preview for FerrisKey IAM."
  />
</svelte:head>

<QueryClientProvider client={queryClient}>
  {@render children()}
</QueryClientProvider>
