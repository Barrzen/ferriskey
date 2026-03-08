<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Compass"
  description="Authentication flow mapping will evolve into an interactive visual builder with stronger readability, safer edits, and better debugging context."
  highlights={[
    'Flow graph canvas',
    'Node-level execution insight',
    'Versioned flow review',
    'Safer staged publishing'
  ]}
/>
