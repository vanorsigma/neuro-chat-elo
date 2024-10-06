<script lang="ts">
  export let onClick = () => {};
</script>

<div on:click={onClick} class="text-on-bg absolute top-2.5 left-2.5 z-50 cursor-pointer">
  <svg width="2em" height="2em" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M4 18L20 18" stroke-width="2" stroke-linecap="round" />
    <path d="M4 12L20 12" stroke-width="2" stroke-linecap="round" />
    <path d="M4 6L20 6" stroke-width="2" stroke-linecap="round" />
  </svg>
</div>

<style lang="postcss">
  svg path {
    stroke: theme(colors.on-bg);
  }
</style>
