<script lang="ts">
  import Leaderboard from '$lib/leaderboard.svelte';
  import Podium from '$lib/podium.svelte';
  import type { RankingInfo } from '$lib/ranks';
  import type { User } from '$lib/user';
  import { onMount } from 'svelte';

  export let isActive = false;
  export let rankingInfo: RankingInfo;

  // TODO: Placeholder function
  async function getRankPage(curr: string, direction: boolean) {
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

<div
  class="flex flex-row justify-center items-center w-full h-full gap-3 {isActive ? '' : 'hidden'}"
>
  <div class="bg-chat rounded-xl flex flex-col items-center gap-2 p-5 h-min">
    <h1 class="text-3xl">Top Chatters</h1>
    {#if topUsers}
      <Podium firstPlace={topUsers[0]} secondPlace={topUsers[1]} thirdPlace={topUsers[2]} />
    {/if}
  </div>

  <div class="bg-chat rounded-xl flex flex-col items-center max-h-[90%] flex-1 p-5">
    <h1 class="text-3xl">Leaderboard</h1>
    <Leaderboard fetchNextPage={getRankPage} />
  </div>
</div>
