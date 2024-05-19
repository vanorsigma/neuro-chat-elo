<!--
As with all things web, there's very likely a better way to do
this. I've tried using sticky, but it works funny with
IntersectionObserver, which I'm relying on to apply different styles
during a float.

Supports lazy loading.

TODO: Find a better way to do this, I'm too smooth-brained
-->

<script lang="ts">
  import RankItem from './rankitem.svelte';

  import { onMount, onDestroy, beforeUpdate, afterUpdate } from 'svelte';

  /* Lazy Fetching Shenanigans */
  /**
   * Fetches the next page. Should be async. On initial load, this function will be called with undefined index and direction backwards (i.e. false). It is the component user's responsibility to handle that.
   *
   * @param {string} curr - The current index to the page
   * @param {number} direction - False to go backwards from the current index. True otherwise.
   * @returns {[any[], string, boolean]} Tuple of the consisting of the fetched data, fetched index and a boolean expressing if there are any indexes after this index (in the direction specified)
   */
  export let fetchNextPage = async (
    curr: string,
    direction: boolean
  ): Promise<[any[], string, boolean]> => {
    console.warn(
      `Leaderboard not correctly configured called! curr: ${curr}, direction: ${direction}`
    );
    return [undefined, undefined, false];
  };
  /** Start fetching when the user reaches this number of elements before / after the end / start of the list */
  export let elementsBeforeFetch = 10;

  // Data loading / unloading related variables
  let currentIndexTop: string = undefined;
  let currentIndexBottom: string = undefined;
  let currentData: any[] = [];
  let expectBefore: boolean = true;
  let expectAfter: boolean = true;
  let fetchMarkerBefore: HTMLElement;
  let fetchMarkerAfter: HTMLElement;
  let fetchObserver: IntersectionObserver;
  $: initialDataLoaded = (currentIndexTop === undefined && currentIndexBottom === undefined);

  async function updateData(direction: boolean) {
    const response = await fetchNextPage(direction ? currentIndexBottom : currentIndexTop, direction);

    if (!direction) {
      currentData = [ ...response[0], ...currentData];
      currentIndexTop = response[1];
      expectBefore = response[2];
    } else {
      currentData = [...currentData, ...response[0]];
      currentIndexBottom = response[1];
      expectAfter = response[2];
    }
  }

  onMount(async () => {
    await updateData(false);

    // NOTE: For initialization purposes
    if (!currentIndexBottom)
      currentIndexBottom = currentIndexTop;
  });

  onMount(() => {
    fetchObserver = new IntersectionObserver(
      async (entries, _) => {
        let topIntersect = false;
        let bottomIntersect = false;

        entries.forEach(entry => {
          topIntersect ||= entry.target === fetchMarkerBefore && entry.isIntersecting;
          bottomIntersect ||= entry.target === fetchMarkerAfter && entry.isIntersecting;
        });

        if (topIntersect && expectBefore) {
          updateData(false);
        } else if (bottomIntersect && expectAfter) {
          updateData(true);
        }
      }, { threshold: 1 });
  });

  beforeUpdate(() => {
    if (!fetchObserver)
      return;

    fetchMarkerBefore ? fetchObserver.unobserve(fetchMarkerBefore) : null;
    fetchMarkerAfter ? fetchObserver.unobserve(fetchMarkerAfter) : null;
  });

  afterUpdate(() => {
    if (!fetchObserver)
      return;

    fetchMarkerBefore ? fetchObserver.observe(fetchMarkerBefore) : null;
    fetchMarkerAfter ? fetchObserver.observe(fetchMarkerAfter) : null;
  })

  onDestroy(() => {
    if (!fetchObserver)
      return;

    fetchObserver.unobserve(fetchMarkerBefore);
    fetchObserver.unobserve(fetchMarkerAfter);
  })

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

  let headerWidth = 0;
  let headerTop = 0;
  let headerLeft = 0;

  onMount(() => {
    headerObserver = new IntersectionObserver(
      async (entries, _) => {
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
      },
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
  })

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
</script>

<div bind:this={containerElement} class="relative w-full h-full overflow-y-scroll">
  <div class="relative w-full" bind:this={parentElement}>
    <div
      bind:this={stickyElement}
      class={leaderboardClasses}
      style="width: {headerWidth}px; top: {headerTop}px; left: {headerLeft}px"
    >
      <b>Rank</b>
      <b>User</b>
      <b>Elo Score</b>
      <b>Delta</b>
    </div>
  </div>
  <div class="grid auto-rows-auto grid-cols-4 w-full">
    {#if currentData.length < elementsBeforeFetch && initialDataLoaded}
      <div bind:this={fetchMarkerBefore} class="invisible col-span-4"></div>
    {/if}

    {#each currentData as rank, i}
      <RankItem rank={rank.rank} score={rank.elo} username={rank.username} delta={rank.delta} />
      {#if i == elementsBeforeFetch}
        <div bind:this={fetchMarkerBefore} class="col-span-4"></div>
      {/if}

      {#if i == currentData.length - elementsBeforeFetch}
        <div bind:this={fetchMarkerAfter} class="col-span-4"></div>
      {/if}
    {/each}

    {#if currentData.length < elementsBeforeFetch && initialDataLoaded}
      <div bind:this={fetchMarkerAfter} class="invisible col-span-4"></div>
    {/if}
  </div>
</div>
