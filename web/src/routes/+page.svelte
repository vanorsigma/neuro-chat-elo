<script lang="ts">
  import RankingCard from './rankingCard.svelte';
  import Carousel from '$lib/carousel.svelte';
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
    <div class="flex flex-col w-full h-[90%] {index === activeIndex ? '' : 'hidden'}">
      <h1 class="text-3xl flex-none font-bold">{rankingTitles[index]}</h1>
      <RankingCard isActive={index === activeIndex} {rankingInfo} />
    </div>
  {/each}
</Carousel>
