<script lang="ts">
  import RevealCard from './revealCard.svelte';
  import type { RevealMetadata } from './revealMetadata';

  let currentIndex = 0;
  export let revealMetadatas: RevealMetadata[] = [];
  export let allAnimationsDone = () => {};

  function onAnimationDone() {
    if (currentIndex + 1 >= revealMetadatas.length) {
      setTimeout(allAnimationsDone, 1000);
    }

    currentIndex += 1;
  }
</script>

<button class="absolute z-50 top-10" on:click={allAnimationsDone}>Skip</button>
{#each revealMetadatas as metadata, index}
  {#if index === currentIndex}
    <RevealCard
      avatarUrl={metadata.avatarUrl}
      avatarName={metadata.avatarName}
      topChatterRevealTitle={`${metadata.leaderboardName} top chatter is...`}
      animationDoneCallback={onAnimationDone}
    />
  {/if}
{/each}
