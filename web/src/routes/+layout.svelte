<script>
  import './page.css';
</script>

<div
  class="relative mx-auto flex flex-col justify-center items-center md:h-screen w-full gap-1 md:gap-5"
>
  <div class="w-full min-h-screen h-full md:min-h-[90vh] md:max-h-[90%]">
    <slot />
  </div>
</div>
