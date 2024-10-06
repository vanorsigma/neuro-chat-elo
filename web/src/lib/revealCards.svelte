<script lang="ts">
  import type { RevealMetadata } from './revealMetadata';
  import RevealCardCollection from './revealCardCollection.svelte';

  let currentTimeout: number | undefined;
  let moveToLeaderboardsBtnTxt = 'Skip';
  export let revealMetadatas: RevealMetadata[] = [];
  export let allAnimationsDone = () => {};

  function onRevealDone() {
    moveToLeaderboardsBtnTxt = 'Continue >>';
  }
</script>

<button
  class="absolute text-on-bg z-50 top-10 left-4"
  on:click={() => {
    allAnimationsDone();
    if (currentTimeout) {
      clearTimeout(currentTimeout);
    }
  }}>{moveToLeaderboardsBtnTxt}</button
>

<RevealCardCollection {onRevealDone} {revealMetadatas} bind:currentTimeout />
