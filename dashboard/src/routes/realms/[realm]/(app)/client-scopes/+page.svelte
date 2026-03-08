<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Client scopes"
  description="Protocol mappers, shared scopes, and claim composition are being redesigned for faster review and less hidden coupling."
  highlights={[
    'Mapper status previews',
    'Diff-friendly editing',
    'Assigned clients visibility',
    'Cleaner scope lifecycle flow'
  ]}
/>
