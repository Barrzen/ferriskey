<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="SeaWatch"
  description="Audit and security insights will become a sharper operations center with incident-first timelines, filters, and export views."
  highlights={[
    'Real-time event timeline',
    'Segmented severity filters',
    'Saved search presets',
    'Export and handoff workflows'
  ]}
/>
