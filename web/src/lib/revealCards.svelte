<script lang="ts">
  import type { RevealMetadata } from './revealMetadata';
  import RevealCardCollection from './revealCardCollection.svelte';

  let currentTimeout: number | undefined;
  export let revealMetadatas: RevealMetadata[] = [];
  export let allAnimationsDone = () => {};
</script>

<button
  class="absolute z-50 top-10"
  on:click={() => {
    allAnimationsDone();
    if (currentTimeout) {
      clearTimeout(currentTimeout);
    }
  }}>Skip</button
>

<RevealCardCollection {revealMetadatas} bind:currentTimeout />
