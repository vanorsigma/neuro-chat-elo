<script lang="ts">
  export let text = '';
  export let onClick = () => {};

  export let selected = false;
</script>

<div on:click={onClick}>
  <p class="item {selected ? 'selected-item bold underline' : 'cursor-pointer'}">{text}</p>
</div>

<style>
  .item {
    color: salmon;
  }
</style>
