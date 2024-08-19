<script>
  import './page.css';
</script>

<div
  class="container mx-auto flex flex-col justify-center items-center md:h-screen w-full gap-1 md:gap-5"
>
  <div class="relative container min-h-screen h-full md:min-h-[90vh] md:max-h-[90%] pb-2 sm:pb-0">
    <slot />
  </div>
  <p class="flex-none text-center hidden sm:block">
    Public Alpha. Please ping @vanorsigma for feedback
  </p>
</div>
