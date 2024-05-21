<script lang="ts">
  import RankingCard from './rankingCard.svelte';
  import Carousel from '$lib/carousel.svelte';
  import { fly } from 'svelte/transition';
  import { ranks, altRanks } from '$lib/ranks';

  let activeIndex = 0;
  let rankingTitles = ['Overall', 'Non-VIPS', 'Only Chatting'];
  let ranking = [$ranks, $altRanks, undefined];

  function navigatePage(offset: number) {
    activeIndex = (activeIndex + offset) % (ranking.length - 1);
    while (activeIndex < 0) {
      activeIndex = ranking.length + activeIndex - 1;
    }
  }
</script>

<Carousel previousPage={() => navigatePage(-1)} nextPage={() => navigatePage(1)}>
  {#each ranking as rankingInfo, index}
    {#if index === activeIndex}
      <div
        in:fly={{ x: -window.innerWidth, duration: 200, delay: 201 }}
        out:fly={{ x: window.innerWidth, duration: 200 }}
        class="flex flex-col w-full h-full md:h-[90%]"
      >
        <h1 class="text-3xl flex-none font-bold my-5 md:my-0 text-center">
          {rankingTitles[index]}
        </h1>
        <RankingCard {rankingInfo} />
      </div>
    {/if}
  {/each}
</Carousel>
