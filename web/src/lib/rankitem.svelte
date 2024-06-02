<script lang="ts">
  export let rank: number = NaN;
  export let username: string = '';
  export let score: number = NaN;
  export let delta: number = NaN;

  export let rankWidth = NaN;
  export let userWidth = NaN;
  export let eloWidth = NaN;
  export let deltaWidth = NaN;
</script>

<p style="width: {rankWidth}px;">{rank}</p>
<p style="width: {userWidth}px;" class="truncate">{username}</p>
<p style="width: {eloWidth}px;" class="collapse sm:visible">{score.toFixed(2)}</p>
<p style="width: {deltaWidth}px;"
   class={delta == 0 ? 'text-yellow-700' : (delta > 0 ? 'text-lime-600' : 'text-red-500')}>
  {delta == 0 ? '' : (delta > 0 ? '▲' : '▼')}{delta}
</p>
