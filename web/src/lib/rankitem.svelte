<script lang="ts">
  export let rank: number = NaN;
  export let username: string = '';
  export let score: number = NaN;
  export let delta: number = NaN;
</script>

<p>{rank}</p>
<p>{username}</p>
<p>{score.toFixed(2)}</p>
<p class={delta > 0 ? 'text-lime-600' : 'text-red-500'}>
  {delta > 0 ? '▲' : '▼'}{delta}
</p>
