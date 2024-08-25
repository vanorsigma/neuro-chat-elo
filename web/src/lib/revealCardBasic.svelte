<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import RevealCard from './revealCard.svelte';

  export let avatarUrl = '';
  export let avatarName = '';
  export let title = '';
  export let winner = false;

  const defaultAvatarUrl =
    'https://static-cdn.jtvnw.net/user-default-pictures-uv/ebe4cd89-b4f4-4cd9-adac-2f30151b4209-profile_image-300x300.png';

  export let onFinishedLoading = () => {};

  onMount(async () => {
    onFinishedLoading();
  });
</script>

<RevealCard
  username={avatarName}
  avatarUrl={avatarUrl ? avatarUrl : defaultAvatarUrl}
  {title}
  {winner}
/>
