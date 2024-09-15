<script lang="ts">
  import Leaderboard from '$lib/leaderboard.svelte';
  import Podium from '$lib/podium.svelte';
  import type { RankingInfo } from '$lib/ranks';
  import type { User } from '$lib/user';
  import { onDestroy, onMount } from 'svelte';

  export let isActive: boolean;
  export let rankingInfo: RankingInfo[];
  export let userSearchTextValue: string;
  var windowWidth = window.innerWidth;
  $: rankingInfoLength = rankingInfo.length;

  let topUsers: User[] | undefined;

  $: {
    topUsers = rankingInfo
      .slice()
      .sort((a, b) => a.rank > b.rank)
      .slice(0, 3)
      .map((data) => ({
        name: data.username,
        elo: data.elo,
        avatar: data.avatar
      }));
  }

  let currentTimeout;
  function onSearchTextValueInput(e: InputEvent) {
    clearTimeout(currentTimeout);
    currentTimeout = setTimeout(() => {
      userSearchTextValue = e.target?.value;
    }, 200);
  }

  function onWindowResize() {
    windowWidth = window.innerWidth;
  }

  onMount(() => {
    window.addEventListener('resize', onWindowResize);
  });

  onDestroy(() => {
    window.removeEventListener('resize', onWindowResize);
  });
</script>

<div class="flex flex-col md:flex-row justify-center items-center w-full h-full gap-3 min-h-0">
  <div class="bg-chat rounded-xl flex flex-col flex-none md:flex-0 items-center gap-2 p-5 h-min">
    <h1 class="text-3xl">Top Chatters</h1>
    {#if rankingInfoLength >= 3}
      <Podium
        size={windowWidth < 768 ? '100%' : '60vmin'}
        firstPlace={topUsers[0]}
        secondPlace={topUsers[1]}
        thirdPlace={topUsers[2]}
      />
    {/if}
  </div>

  <div
    class="bg-chat rounded-xl flex flex-col items-center max-h-[70vh] md:max-h-[90%] flex-0 p-5 w-full md:w-[40%] min-h-0"
  >
    <h1 class="text-3xl">Leaderboard</h1>
    <input
      class="md:self-end m-2"
      type="text"
      placeholder="Search username..."
      alt="Username"
      on:input={(e) => onSearchTextValueInput(e)}
      value={userSearchTextValue}
    />
    {#if rankingInfoLength >= 3}
      <Leaderboard {isActive} searchTerm={userSearchTextValue} currentData={rankingInfo} />
    {/if}
  </div>
</div>
