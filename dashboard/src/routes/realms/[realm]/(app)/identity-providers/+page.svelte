<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Identity providers"
  description="SSO provider setup will gain better connection state, attribute mapping guidance, and clearer troubleshooting compared with the current frontend."
  highlights={[
    'Provider status cards',
    'Connection test flow',
    'Mapping hint panels',
    'Safer alias editing'
  ]}
/>
