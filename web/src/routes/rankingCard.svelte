<script lang="ts">
  import { sanitizeString } from '$lib';
  import Leaderboard from '$lib/leaderboard.svelte';
  import Podium from '$lib/podium.svelte';
  import type { RankingInfo } from '$lib/ranks';
  import type { User } from '$lib/user';

  export let isActive: boolean;
  export let rankingInfo: RankingInfo[];
  export let userSearchTextValue: string;
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
</script>

<div class="flex flex-col md:flex-row justify-center items-center w-full h-full gap-3 min-h-0">
  <div class="bg-chat rounded-xl flex flex-col flex-none md:flex-1 items-center gap-2 p-5 h-min">
    <h1 class="text-3xl">Top Chatters</h1>
    {#if rankingInfoLength >= 3}
      <Podium
        scaleToX={window.innerWidth < 768 ? window.innerWidth * 0.65 : 400}
        firstPlace={topUsers[0]}
        secondPlace={topUsers[1]}
        thirdPlace={topUsers[2]}
      />
    {/if}
  </div>

  <div
    class="bg-chat rounded-xl flex flex-col items-center md:max-h-[90%] flex-auto p-5 w-full md:w-auto min-h-0"
  >
    <h1 class="text-3xl">Leaderboard</h1>
    <input
      class="md:self-end m-2"
      type="text"
      placeholder="Search username..."
      alt="Username"
      bind:value={userSearchTextValue}
    />
    {#if rankingInfoLength > 3}
      <Leaderboard {isActive} searchTerm={userSearchTextValue} currentData={rankingInfo} />
    {/if}
  </div>
</div>
