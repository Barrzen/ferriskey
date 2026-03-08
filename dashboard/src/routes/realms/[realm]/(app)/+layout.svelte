<script lang="ts">
  import AppShell from '$components/AppShell.svelte';
  import { page } from '$app/state';

  let { children } = $props();
  const currentRealm = $derived(String(page.params.realm ?? 'master'));
</script>

<AppShell realm={currentRealm}>{@render children()}</AppShell>
