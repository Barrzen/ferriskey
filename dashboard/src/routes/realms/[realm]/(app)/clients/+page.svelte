<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Clients"
  description="The new client workspace will organize secrets, redirect URIs, protocol mappers, and role assignments into clearer sections with safer defaults."
  highlights={[
    'Safer secret rotation flows',
    'Redirect URI review patterns',
    'Grant type visibility',
    'Protocol mapper composition'
  ]}
/>
