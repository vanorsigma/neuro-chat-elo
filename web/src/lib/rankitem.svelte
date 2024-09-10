<script lang="ts">
  import Gold from './rankitems/gold.svelte';
  import Silver from './rankitems/silver.svelte';
  import Bronze from './rankitems/bronze.svelte';
  import Normal from './rankitems/normal.svelte';

  // Special flags
  import GoldIronmouse from './rankitems/ironmouse_gold.svelte';
  import SilverIronmouse from './rankitems/ironmouse_silver.svelte';
  import BronzeIronmouse from './rankitems/ironmouse_bronze.svelte';
  import NormalIronmouse from './rankitems/ironmouse_normal.svelte';

  import type { Badge } from './ranks';

  export let rank: number = NaN;
  export let username: string = '';
  export let score: number = NaN;
  export let avatarUrl: string = '';

  // Special flags
  export let special_ironmouse: boolean = false;

  /* TODO: probably implement this in the SVG */
  export let delta: number = NaN;
  export let badges: Badge[];
</script>

{#if rank == 1}
  {#if !special_ironmouse}
    <Gold
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {:else}
    <GoldIronmouse
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {/if}
{:else if rank == 2}
  {#if !special_ironmouse}
    <Silver
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {:else}
    <SilverIronmouse
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {/if}
{:else if rank == 3}
  {#if !special_ironmouse}
    <Bronze
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {:else}
    <BronzeIronmouse
      {avatarUrl}
      {username}
      eloScore={score}
      iconUrl={badges.length > 0 ? badges[0].image_url : ''}
    />
  {/if}
{:else if !special_ironmouse}
  <Normal
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{:else}
  <NormalIronmouse
    {avatarUrl}
    {rank}
    {username}
    eloScore={score}
    iconUrl={badges.length > 0 ? badges[0].image_url : ''}
  />
{/if}
