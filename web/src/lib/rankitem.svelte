<script lang="ts">
  import type { Badge } from './ranks';

  export let rank: number = NaN;
  export let username: string = '';
  export let score: number = NaN;
  export let delta: number = NaN;
  export let badges: Badge[];

  export let rankWidth = 0;
  export let userWidth = 0;
  export let eloWidth = 0;
  export let deltaWidth = 0;

  let usernameEle: HTMLParagraphElement;
  let badgeHeights = 0;
  $: {
    // react to userWidth too
    userWidth;
    badgeHeights = usernameEle?.clientHeight;
  }
</script>

<p style="width: {rankWidth}px;">{rank}</p>
<div class="flex flex-row gap-1 truncate" style="width: {userWidth}px">
  <p class="w-fit min-w-fit max-w-full h-fit truncate shrink-0 grow-1" bind:this={usernameEle}>
    {username}
  </p>
  <div class="flex flex-row flex-0 gap-1">
    {#each badges as badge}
      <img
        class="inline relative"
        src={badge.image_url}
        alt={badge.description}
        style="height: {badgeHeights}px; width: {badgeHeights}px"
        title={badge.description}
      />
    {/each}
  </div>
</div>
<p style="width: {eloWidth}px;" class="collapse truncate sm:visible">{score.toFixed(2)}</p>
<p
  style="width: {deltaWidth}px;"
  class={delta == 0 ? 'text-yellow-700' : delta > 0 ? 'text-lime-600' : 'text-red-500'}
>
  {delta == 0 ? '' : delta > 0 ? '▲' : '▼'}{delta}
</p>
