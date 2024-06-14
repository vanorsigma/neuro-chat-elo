<!--
Leaderboard.

NOTE: The original design supported lazy loading, but after some
visual struggles and careful thought, I realized that it wasn't really
needed, so it no longer supports lazy loading. Please refer to commit
1371382 at your own risk
-->

<script lang="ts">
  import RankItem from './rankitem.svelte';

  import { onMount, onDestroy, afterUpdate } from 'svelte';
  import type { RankingInfo } from './ranks';

  export let currentData: RankingInfo[];
  export let isActive: boolean;
  let lagIsActive: boolean = false;

  /* Header Rendering Shenanigans */
  // Leaderboard header rendering related variables
  const leaderboardHeaderBase = 'text-semibold grid grid-row-1 grid-cols-4 top-0 w-full';
  const leaderboardHeaderFloat = ' fixed bg-gray-200';
  const leaderboardHeaderHide = ' invisible';
  let leaderboardClasses = leaderboardHeaderBase + leaderboardHeaderFloat;
  let containerElement: HTMLDivElement;
  let parentElement: HTMLDivElement;
  let stickyElement: HTMLDivElement;
  let headerObserver: IntersectionObserver;

  let rankElement: HTMLParagraphElement | undefined;
  let userElement: HTMLParagraphElement | undefined;
  let eloElement: HTMLParagraphElement | undefined;
  let deltaElement: HTMLParagraphElement | undefined;

  let rankWidth = rankElement?.clientWidth;
  let userWidth = userElement?.clientWidth;
  let eloWidth = eloElement?.clientWidth;
  let deltaWidth = deltaElement?.clientWidth;
  $: setTimeout(() => {
    isActive; // create dependency, so that widths get updated
    rankWidth = rankElement?.clientWidth == 0 ? rankWidth : rankElement?.clientWidth;
    userWidth = userElement?.clientWidth == 0 ? userWidth : userElement?.clientWidth;
    eloWidth = eloElement?.clientWidth == 0 ? eloWidth : eloElement?.clientWidth;
    deltaWidth = deltaElement?.clientWidth == 0 ? deltaWidth : deltaElement?.clientWidth;
  }, 0);

  let headerWidth = 0;
  let headerTop = 0;
  let headerLeft = 0;

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
    // TODO: This probably affects performance a lot. Probably need to make it faster
    let isContainerIntersect = isInView(containerElement);

    entries.forEach((entry) => {
      isStickyIntersect ||= entry.target == stickyElement && entry.isIntersecting;
      isParentIntersect ||= entry.target == parentElement && entry.isIntersecting;
      // isContainerIntersect ||= entry.target == containerElement && entry.isIntersecting;
    });

    if (!isStickyIntersect && !isParentIntersect) {
      leaderboardClasses = leaderboardHeaderBase + leaderboardHeaderFloat;
    } else if (isParentIntersect) {
      leaderboardClasses = leaderboardHeaderBase;
    }

    if (!isContainerIntersect) {
      leaderboardClasses += leaderboardHeaderHide;
    }

    updateHeaderValues();
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

  onMount(() => {
    headerObserver = new IntersectionObserver(
      (entries) =>
        intersectionCallback(
          entries.map(
            (entry) =>
              ({
                isIntersecting: entry.isIntersecting,
                target: entry.target
              }) as LeaderboardIntersectionEntry
          )
        ),
      { threshold: [0, 1] }
    );

    headerObserver.observe(stickyElement);
    headerObserver.observe(parentElement);
    headerObserver.observe(containerElement);
  });

  onMount(() => {
    window.addEventListener('scroll', updateHeaderValues);

    return () => {
      window.removeEventListener('scroll', updateHeaderValues);
    };
  });

  onDestroy(() => {
    if (!headerObserver) return;

    headerObserver.unobserve(stickyElement);
    headerObserver.unobserve(parentElement);
    headerObserver.unobserve(containerElement);
  });

  function updateHeaderValues() {
    if (!containerElement || !parentElement || !stickyElement) return;

    const containerBoundingRect = containerElement.getBoundingClientRect();

    headerWidth = parentElement.clientWidth;
    // headerTop = containerBoundingRect.top + window.scrollY;
    headerTop = containerBoundingRect.top;
    headerLeft = containerBoundingRect.left + window.scrollX;
  }

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
    return new RegExp(searchTerm, 'i').test(val.username);
  });
</script>

<div bind:this={containerElement} class="relative w-full h-60 grow md:h-full overflow-y-scroll">
  <div class="relative w-full z-50" bind:this={parentElement}>
    <div
      bind:this={stickyElement}
      class={leaderboardClasses}
      style="width: {headerWidth}px; top: {headerTop}px; left: {headerLeft}px"
    >
      <b bind:this={rankElement}>Rank</b>
      <b bind:this={userElement}>User</b>
      <b bind:this={eloElement} class="collapse sm:visible">Elo Score</b>
      <b bind:this={deltaElement}>Delta</b>
    </div>
  </div>
  <div class="grid auto-rows-auto grid-cols-4 w-full">
    {#each filteredList as rank, i}
      <RankItem
        rank={rank.rank}
        score={rank.elo}
        username={rank.username}
        delta={rank.delta}
        badges={rank.badges}
        {rankWidth}
        {userWidth}
        {eloWidth}
        {deltaWidth}
      />
    {/each}
  </div>
</div>
