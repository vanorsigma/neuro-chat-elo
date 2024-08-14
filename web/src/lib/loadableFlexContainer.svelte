<!-- A loadable flex container. This guy exists solely because of the
onMount call, which is useful to show loading screens until things
have fully loaded. -->

<script lang="ts">
  import { onMount } from 'svelte';
  export let onload: () => void = () => {};

  onMount(() => {
    onload();
  });
</script>

<div
  class="container flex flex-row justify-center items-center flex-1 w-full h-[60em] md:h-full gap-3"
>
  <slot />
</div>
