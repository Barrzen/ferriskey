<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Roles"
  description="Realm and client roles will move into a cleaner permission workspace with better hierarchy scanning and impact visibility."
  highlights={[
    'Bitmask-aware permission explanation',
    'Role membership previews',
    'Client-role split views',
    'Safer destructive actions'
  ]}
/>
