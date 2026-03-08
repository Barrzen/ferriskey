<script lang="ts">
  import { page } from '$app/state';
  import PagePlaceholder from '$components/PagePlaceholder.svelte';
</script>

<PagePlaceholder
  realm={String(page.params.realm ?? 'master')}
  title="Realm settings"
  description="Branding, login policies, token behavior, and admin defaults will be consolidated into guided settings panels with stronger explanation."
  highlights={[
    'Token policy sections',
    'Brand preview surfaces',
    'Safer login setting toggles',
    'Feature-gated advanced controls'
  ]}
/>
