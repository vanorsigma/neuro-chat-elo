<script lang="ts">
  import { onMount } from 'svelte';
  import canvasConfetti from 'canvas-confetti';
  import { fade } from 'svelte/transition';
  let canvas: HTMLCanvasElement;
  let confettiCanvas: HTMLCanvasElement;
  let offscreen: OffscreenCanvas;

  let drumAudio: HTMLAudioElement;
  let yippieAudio: HTMLAudioElement;

  let titleHide = false;
  let offscreenDone = false;

  let maxAvatarWidth = NaN;
  const textSize = 30;

  export let avatar_url =
    'https://64.media.tumblr.com/d55402ac7df5b658290fca647b1d4300/fa59626f4dee859b-14/s1280x1920/b86af79b7490871b668274dde83532bfc81d434b.jpg';
  export let avatar_name = 'vanorsigma';

  function drawConfetti(canvas: HTMLCanvasElement) {
    canvasConfetti.create(canvas, {
      resize: true
    })({
      particleCount: 300,
      spread: 160,
      origin: { x: 0.5, y: 0.5 }
    });
  }

  function drawOffscreen(): Promise<OffscreenCanvasRenderingContext2D> {
    const ctx = offscreen.getContext('2d');
    if (!ctx) {
      throw new Error('2d context not supported');
    }

    const img = new Image();
    img.src = avatar_url;
    return new Promise((resolve) => {
      img.onload = () => {
        ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, maxAvatarWidth, maxAvatarWidth);
        ctx.textAlign = 'center';
        ctx.font = '30px arial bold';
        ctx.fillText(avatar_name, maxAvatarWidth * 0.5, maxAvatarWidth + textSize / 2 + 10);
        offscreenDone = true;
        resolve(ctx);
      };
    });
  }

  onMount(async () => {
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      throw new Error('2d context not supported');
    }

    maxAvatarWidth = canvas.clientWidth;
    canvas.width = maxAvatarWidth;
    canvas.height = maxAvatarWidth + textSize / 2 + textSize;

    offscreen = new OffscreenCanvas(maxAvatarWidth, maxAvatarWidth + textSize / 2 + textSize);
    await drawOffscreen();
    drumAudio.play();
    setTimeout(() => {
      titleHide = true;
      setTimeout(() => {
        yippieAudio.play();
        ctx.drawImage(offscreen, 0, 0);
        drawConfetti(confettiCanvas);
      }, 500);
    }, 4000);
  });
</script>

<div class="flex container h-full items-center justify-center">
  {#if !offscreenDone}
    <p>Loading...</p>
  {/if}

  {#if offscreenDone && !titleHide}
    <p transition:fade={{ duration: 200 }} class="text-4xl">Overall Top Chatter is...</p>
  {/if}

  <canvas
    bind:this={canvas}
    in:fade={{ duration: 200, delay: 1000 }}
    class="absolute w-[60%] md:w-[20%] {offscreenDone ? '' : 'invisible'}"
  ></canvas>
  <canvas
    bind:this={confettiCanvas}
    in:fade={{ duration: 200, delay: 200 }}
    class="absolute w-full h-full {offscreenDone ? '' : 'invisible'}"
  ></canvas>
  <audio bind:this={yippieAudio} autoplay={false} muted={false} src="./audio/yippie.mp3"></audio>
  <audio bind:this={drumAudio} autoplay={false} muted={false} src="./audio/drum.mp3"></audio>
</div>
