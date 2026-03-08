<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Users"
  description="Account search, profile editing, required actions, role mapping, and credential workflows will land here with richer visibility than the legacy UI."
  highlights={[
    'Advanced table density controls',
    'Activity-aware profile drawer',
    'Credential and MFA sections',
    'Faster bulk actions and review states'
  ]}
/>
