<script lang="ts">
  import RankingCard from './rankingCard.svelte';
  import Carousel from '$lib/carousel.svelte';
  import { fly } from 'svelte/transition';
  import { overallRank, chatOnlyRank, copypastaRank, nonvipsRank } from '$lib/ranks';
  import { sanitizeString } from '$lib';
  import { tweened } from 'svelte/motion';

  let activeIndex =
    Number(sanitizeString(new URL(window.location.href).searchParams.get('index'))) || 0;
  let rankingTitles = ['Overall', 'Non-VIPS', 'Only Chat Messages', 'Copypasta Leaders'];
  $: ranking = [$overallRank, $nonvipsRank, $chatOnlyRank, $copypastaRank];

  function navigatePage(offset: number) {
    activeIndex = (activeIndex + offset) % ranking.length;
    while (activeIndex < 0) {
      activeIndex = ranking.length + activeIndex;
    }
  }

  $: {
    const url = new URL(window.location.href);
    url.searchParams.set('index', activeIndex.toString());
    // HACK: I get an error when trying to use Svelte's replaceState,
    // so this'll do for now
    window.history.replaceState({}, '', url.toString());}

</script>

<Carousel previousPage={() => navigatePage(-1)} nextPage={() => navigatePage(1)}>
  {#each ranking as rankingInfo, index}
    <div
      in:fly={{ x: -window.innerWidth, duration: 200, delay: 201 }}
      out:fly={{ x: window.innerWidth, duration: 200 }}
      class="flex flex-col w-full h-full md:h-[90%] {index === activeIndex ? '' : 'hidden'}"
      >
      <h1 class="text-3xl flex-none font-bold my-5 md:my-0 text-center">
        {rankingTitles[index]}
      </h1>
      <RankingCard {rankingInfo} />
    </div>
  {/each}
</Carousel>
