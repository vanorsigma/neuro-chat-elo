<script lang="ts">
  import MenuItem from './menuitem.svelte';
  import { fly } from 'svelte/transition';

  const twitchMenuItemMapping = {
    Overall: 0,
    'Non-VIPS': 1,
    'Chat Only': 2,
    Copypasta: 3,
    Bits: 4,
    Subs: 5
  };

  export let itemClicked: (arg0: number) => void;
  export let selectedPage: number;

  function onMenuItemClick(page: number) {
    const searchParams = new URLSearchParams(window.location.search);
    searchParams.set('index', `${page}`);
    window.history.pushState(
      {},
      '',
      window.location.origin + window.location.pathname + '?' + searchParams.toString()
    );
    itemClicked(page);
  }
</script>

<div
  transition:fly={{ x: -100 }}
  class="text-center absolute top-0 left-0 py-10 h-full w-6/12 md:w-72 bg-chat"
>
  <p class="text-4xl">Menu</p>
  <br />

  <p class="text-xl">Twitch</p>
  {#each Object.entries(twitchMenuItemMapping) as menuItem}
    <MenuItem
      text={menuItem[0]}
      onClick={() => onMenuItemClick(menuItem[1])}
      selected={selectedPage == menuItem[1]}
    />
  {/each}
</div>
