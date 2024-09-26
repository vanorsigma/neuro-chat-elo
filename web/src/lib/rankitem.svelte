<script lang="ts">
  import Gold from './rankitems/gold.svelte';
  import Silver from './rankitems/silver.svelte';
  import Bronze from './rankitems/bronze.svelte';
  import Normal from './rankitems/normal.svelte';
  import type { Badge } from './ranks';

  export let rank: number = NaN;
  export let username: string = '';
  export let score: number = NaN;
  export let avatarUrl: string = '';

  /* TODO: probably implement this in the SVG */
  export let delta: number = NaN;
  export let badges: Badge[];
</script>

{#if rank == 1}
  <Gold
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{:else if rank == 2}
  <Silver
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{:else if rank == 3}
  <Bronze
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{:else}
  <Normal
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{/if}
