<script lang="ts">
  export let previousPage: () => void;
  export let nextPage: () => void;
</script>

<div class="container flex flex-row justify-center items-center flex-1 w-full h-full gap-3">
  <button class="text-5xl button" aria-label="Previous Leaderboard" on:click={previousPage}
    >&lt;</button
  >
  <slot />
  <button class="text-5xl button" aria-label="Next Leaderboard" on:click={nextPage}>&gt;</button>
</div>
