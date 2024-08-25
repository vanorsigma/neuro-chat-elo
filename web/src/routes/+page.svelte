<script lang="ts">
  import RankingCard from '$lib/rankingCard.svelte';
  import LoadableFlexContainer from '$lib/loadableFlexContainer.svelte';
  import RevealCards from '$lib/revealCards.svelte';
  import type { RevealMetadata } from '$lib/revealMetadata';
  import {
    overallRank,
    chatOnlyRank,
    copypastaRank,
    nonvipsRank,
    bitsRank,
    subsRank,
    discordRank,
    partnersRank,
    bilibiliRank
  } from '$lib/ranks';
  import { sanitizeString } from '$lib';
  import Menu from '$lib/menu.svelte';
  import Burger from '$lib/burger.svelte';
  import { onMount } from 'svelte';

  let showRankingsLoading = false;
  let allowRankings = false; // this forces the loading text to appear
  let activeIndex =
    Number(sanitizeString(new URL(window.location.href).searchParams.get('index'))) || 0;
  let rankingTitles = [
    'Overall',
    'Non-VIPS',
    'Only Chat Messages',
    'Copypasta Leaders',
    'Bits',
    'Subs',
    'Partners',
    '#livestream-chat',
    'bilibili'
  ];
  $: ranking = [
    $overallRank,
    $nonvipsRank,
    $chatOnlyRank,
    $copypastaRank,
    $bitsRank,
    $subsRank,
    $partnersRank,
    $discordRank,
    $bilibiliRank
  ];

  $: allRanksLoaded = ranking.every((r) => r.length > 0);

  let menuAppear = false;

  function navigatePage(page: number) {
    activeIndex = page;
  }

  $: {
    const url = new URL(window.location.href);
    url.searchParams.set('index', activeIndex.toString());
    // HACK: I get an error when trying to use Svelte's replaceState,
    // so this'll do for now
    window.history.replaceState({}, '', url.toString());
  }

  // Searching shenanigans
  let userSearchTextValue: string = new URL(window.location.href).searchParams.get('search') || '';

  $: {
    const url = new URL(window.location.href);
    if (userSearchTextValue === undefined || userSearchTextValue === '') {
      url.searchParams.set('search', '');
    } else {
      url.searchParams.set('search', sanitizeString(userSearchTextValue));
    }
    window.history.replaceState({}, '', url.toString());
  }

  // Reveal Shenanigans
  async function onAnimationDone() {
    showRankingsLoading = true;

    // Give some time for the loading to show up
    setTimeout(() => {
      allowRankings = true;
    }, 100);
  }

  $: metadatas = ranking.map((rankingInfo: RankingInfo[], idx) => {
    return {
      avatarName: rankingInfo[0]?.username,
      avatarUrl: rankingInfo[0]?.avatar,
      leaderboardName: rankingTitles[idx]
    } as RevealMetadata;
  });
</script>

<svg width="0" height="0">
  <defs>
    <linearGradient x1="0%" y1="0%" x2="100%" y2="0%" id="mx-gradient-ffd700-1-ffb570-1-e-0">
      <stop offset="0%" style="stop-color: rgb(255, 215, 0); stop-opacity: 1;" />
      <stop offset="100%" style="stop-color: rgb(255, 181, 112); stop-opacity: 1;" />
    </linearGradient>

    <linearGradient x1="0%" y1="0%" x2="100%" y2="0%" id="mx-gradient-4d4d4d-1-c0c0c0-1-e-0">
      <stop offset="0%" style="stop-color: rgb(77, 77, 77); stop-opacity: 1;" />
      <stop offset="100%" style="stop-color: rgb(192, 192, 192); stop-opacity: 1;" />
    </linearGradient>

    <linearGradient x1="0%" y1="0%" x2="100%" y2="0%" id="mx-gradient-613e00-1-ffb570-1-e-0">
      <stop offset="0%" style="stop-color: rgb(97, 62, 0); stop-opacity: 1;" />
      <stop offset="100%" style="stop-color: rgb(255, 181, 112); stop-opacity: 1;" />
    </linearGradient>

    <linearGradient x1="0%" y1="0%" x2="0%" y2="100%" id="mx-gradient-ffd700-1-d4c11e-1-s-0">
      <stop offset="0%" style="stop-color: rgb(255, 215, 0); stop-opacity: 1;" />
      <stop offset="100%" style="stop-color: rgb(212, 193, 30); stop-opacity: 1;" />
    </linearGradient>
  </defs>
</svg>

{#if showRankingsLoading || !allRanksLoaded}
  <p class="absolute">Loading...</p>
{/if}

{#if allowRankings && allRanksLoaded}
  <Burger
    onClick={() => {
      menuAppear = !menuAppear;
    }}
  />
  {#if menuAppear}
    <Menu
      itemClicked={(page) => {
        navigatePage(page);
        menuAppear = !menuAppear;
      }}
      selectedPage={activeIndex}
    />
  {/if}
  <LoadableFlexContainer
    onload={() => {
      showRankingsLoading = false;
    }}
  >
    {#each ranking as rankingInfo, index}
      <div
        class="flex flex-col px-5 w-full h-full md:h-full md:h-[90%] {index === activeIndex
          ? ''
          : 'hidden'}"
      >
        <h1 class="text-3xl flex-none font-bold my-5 md:my-0 text-center">
          {rankingTitles[index]}
        </h1>
        <RankingCard isActive={index === activeIndex} bind:userSearchTextValue {rankingInfo} />
      </div>
    {/each}
  </LoadableFlexContainer>
{/if}

{#if !showRankingsLoading && !allowRankings && ranking[0]?.length > 0 && allRanksLoaded}
  <RevealCards revealMetadatas={metadatas} allAnimationsDone={onAnimationDone} />
{/if}
