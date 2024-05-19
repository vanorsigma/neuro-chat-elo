<script lang="ts">
  import Leaderboard from '$lib/leaderboard.svelte';
  import Podium from '$lib/podium.svelte';
  import { ranks } from '$lib/ranks';

  // TODO: Placeholder function
  async function getRankPage(curr: string, direction: boolean) {
    const ranksLength = $ranks.length;
    const indexNow = (curr === undefined) ? 0 : (Number(curr));
    const offset = (!direction ? -1 : 1);
    const indexToFetch = Math.max(1, indexNow + offset);
    const shouldFetchMore = !direction ? (indexNow * 30 != 0) :
          (indexToFetch * 30 < ranksLength);
    return [$ranks.slice(direction ? indexToFetch * 30 : (indexToFetch - 1) * 30,
                         direction ? (indexToFetch + 1) * 30 : indexToFetch * 30),
            `${indexToFetch + offset}`,
            shouldFetchMore];
  }
</script>

<h1 class="text-3xl">Podium</h1>
<Podium />

<!-- <h1 class="text-3xl">Leaderboard #1</h1> -->
<!-- <Leaderboard fetchNextPage={getRankPage} /> -->

<!-- <h1 class="text-3xl">Leaderboard #2</h1> -->
<!-- <Leaderboard fetchNextPage={getRankPage} /> -->
