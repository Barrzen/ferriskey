<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="User federation"
  description="External directory integration is planned as a more guided setup journey with sync visibility, health checks, and reconciliation tooling."
  highlights={[
    'Directory sync monitoring',
    'Mapper validation states',
    'Import and sync strategy view',
    'Clear test connection actions'
  ]}
/>
