<script>
  import './page.css';
</script>

<div class="container mx-auto flex flex-col justify-center items-center h-screen w-full gap-5">
  <div class="container max-h-[90%]">
    <slot />
  </div>
  <p class="flex-none">Public Alpha. Please ping @vanorsigma for feedback</p>
</div>
