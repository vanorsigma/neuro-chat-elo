<!--Leaderboard.-->

<script lang="ts">
  import RankItem from './rankitem.svelte';

  import { onMount, afterUpdate } from 'svelte';
  import { type RankingInfo, ironmousePixelRank } from './ranks';

  export let currentData: RankingInfo[];
  export let isActive: boolean;
  let lagIsActive: boolean = false;

  /* Header Rendering Shenanigans */
  // Leaderboard header rendering related variables
  let containerElement: HTMLDivElement;
  let parentElement: HTMLDivElement;
  let stickyElement: HTMLDivElement;

  $: setTimeout(() => {
    isActive; // create dependency, so that widths get updated
  }, 0);

  interface LeaderboardIntersectionEntry {
    readonly target: HTMLElement;
    readonly isIntersecting: boolean;
  }

  async function intersectionCallback(entries: LeaderboardIntersectionEntry[]) {
    let isStickyIntersect = false;
    let isParentIntersect = false;

    // NOTE: IntersectionObserver doesn't report the correct intersect
    // when the page first loads, so instead of relying on the value from
    // IntersectionObserver, we use the value here.
    entries.forEach((entry) => {
      isStickyIntersect ||= entry.target == stickyElement && entry.isIntersecting;
      isParentIntersect ||= entry.target == parentElement && entry.isIntersecting;
    });
  }

  afterUpdate(() => {
    if (isActive !== lagIsActive && parentElement && containerElement) {
      intersectionCallback(
        [parentElement, containerElement].map((entry) => ({
          isIntersecting: isInView(entry),
          target: entry
        }))
      );
      lagIsActive = isActive;
    }
  });

  function isInView(element: HTMLElement) {
    const rect = element.getBoundingClientRect();

    return (
      rect.top < (window.innerHeight || document.documentElement.clientHeight) &&
      rect.bottom > 0 &&
      rect.left < (window.innerWidth || document.documentElement.clientWidth) &&
      rect.right > 0
    );
  }

  /* Searchable Shenanigans */
  export let searchTerm = '';
  $: filteredList = currentData.filter((val) => {
    // every time this has to run, we reset the slice end
    sliceEnd = defaultEnd;
    return new RegExp(searchTerm, 'i').test(val.username);
  });

  /* Lazy Loading */
  // NOTE: Browsers are really good at loading large data files into memory,
  // but displaying them on the DOM is a completely different story
  const defaultEnd = 50;
  let sliceEnd = defaultEnd;
  let endMarker: HTMLDivElement;

  onMount(() => {
    const intersectionObserver = new IntersectionObserver(
      (entries) => {
        const isVisible = entries.every((val) => val.isIntersecting);

        if (isVisible && sliceEnd < filteredList.length) {
          sliceEnd += 50;
        }
      },
      { threshold: 0.1 }
    );
    intersectionObserver.observe(endMarker);

    return () => {
      intersectionObserver.disconnect();
    };
  });

  /* Special Events */
  $: specialIronmouseLookup = new Set($ironmousePixelRank.map((rank: RankingInfo) => rank.id));
</script>

<div bind:this={containerElement} class="relative w-full md:h-60 grow md:h-full overflow-y-scroll">
  <div class="w-full">
    {#each filteredList.slice(0, sliceEnd) as rank}
      <RankItem
        rank={rank.rank}
        score={rank.elo}
        username={rank.username}
        delta={rank.delta}
        avatarUrl={rank.avatar}
        badges={rank.badges == null ? [] : rank.badges}
        special_ironmouse={specialIronmouseLookup.has(rank.username)}
      />
    {/each}
    <div class="h-1" bind:this={endMarker} />
  </div>
</div>
