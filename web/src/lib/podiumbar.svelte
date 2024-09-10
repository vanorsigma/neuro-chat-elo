<script lang="ts">
  export let x = 100;
  export let scaling = 1.0;
  export let color = '#c0c0c0';

  $: height = 180 * scaling;
  $: y = 121 + (180 - height);
</script>

<rect {x} {y} width="100" {height} fill={color} stroke="rgb(0, 0, 0)" pointer-events="all" />
