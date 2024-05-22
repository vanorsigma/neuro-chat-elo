<script lang="ts">
  import Leaderboard from '$lib/leaderboard.svelte';
  import Podium from '$lib/podium.svelte';
  import type { RankingInfo } from '$lib/ranks';
  import type { User } from '$lib/user';
  import { onMount } from 'svelte';

  export let rankingInfo: RankingInfo;
  let podiumCard: HTMLDivElement;

  // TODO: Placeholder function. This placeholder function attempts to emulate
  // multi-page data, but I figured it wasn't worth it anymore
  async function _legacy_getRankPage(curr: string, direction: boolean) {
    const ranksLength = rankingInfo.length;
    const indexNow = curr === undefined ? 0 : Number(curr);
    const offset = !direction ? -1 : 1;
    const indexToFetch = Math.max(1, indexNow + offset);
    const shouldFetchMore = !direction ? indexNow * 30 != 0 : indexToFetch * 30 < ranksLength;
    return [
      rankingInfo.slice(
        direction ? indexToFetch * 30 : (indexToFetch - 1) * 30,
        direction ? (indexToFetch + 1) * 30 : indexToFetch * 30
      ),
      `${indexToFetch + offset}`,
      shouldFetchMore
    ];
  }

  function getRankPage(_curr: string, _direction: boolean) {
    return [rankingInfo, '1', false];
  }

  async function getTopUsers(): Promise<User[]> {
    return rankingInfo
      .slice()
      .sort((a, b) => a.rank > b.rank)
      .slice(0, 3)
      .map((data) => ({
        name: data.username,
        elo: data.elo,
        avatar: data.avatar
      }));
  }

  let topUsers: User[] | undefined;

  onMount(async () => {
    topUsers = await getTopUsers();
  });
</script>

<div class="flex flex-col md:flex-row justify-center items-center w-full h-full gap-3">
  <div
    bind:this={podiumCard}
    class="bg-chat rounded-xl flex flex-col flex-none md:flex-1 items-center gap-2 p-5 h-min"
  >
    <h1 class="text-3xl">Top Chatters</h1>
    {#if topUsers}
      <Podium
        scaleToX={window.innerWidth < 768 ? podiumCard.clientWidth : 400}
        firstPlace={topUsers[0]}
        secondPlace={topUsers[1]}
        thirdPlace={topUsers[2]}
      />
    {/if}
  </div>

  <div
    class="bg-chat rounded-xl flex flex-col items-center md:max-h-[90%] flex-auto p-5 w-full md:w-auto"
  >
    <h1 class="text-3xl">Leaderboard</h1>
    <Leaderboard fetchNextPage={getRankPage} />
  </div>
</div>
