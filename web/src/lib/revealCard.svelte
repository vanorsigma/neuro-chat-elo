<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
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
  let currentTimeout = NaN;
  const textSize = 30;
  const titleShowTime = 500;
  const avatarShowTime = 2000;
  const revealTime = 3000;

  export let avatarUrl = '';
  export let avatarName = '';
  export let topChatterRevealTitle = 'he forgor to fill this in';
  export let animationDoneCallback = () => {};

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
    img.src = avatarUrl;
    return new Promise((resolve) => {
      img.onload = () => {
        const offset = (canvas.width - maxAvatarWidth) * 0.5;
        ctx.drawImage(img, 0, 0, img.width, img.height, offset, 0, maxAvatarWidth, maxAvatarWidth);
        ctx.textAlign = 'center';
        ctx.font = `${textSize}px arial bold`;
        ctx.fillText(avatarName, canvas.width * 0.5, maxAvatarWidth + textSize / 2 + 10);
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
    canvas.width = Math.max(maxAvatarWidth, avatarName.length * textSize);
    canvas.height = maxAvatarWidth + textSize / 2 + textSize;

    offscreen = new OffscreenCanvas(canvas.width, canvas.height);
    await drawOffscreen();
    drumAudio.play();

    // fire and forget, surely it will be fine
    currentTimeout = setTimeout(() => {
      titleHide = true;
      currentTimeout = setTimeout(() => {
        drumAudio.pause();
        yippieAudio.play();
        ctx.drawImage(offscreen, 0, 0);
        drawConfetti(confettiCanvas);
        currentTimeout = setTimeout(() => {
          yippieAudio.pause();
          animationDoneCallback();
        }, revealTime);
      }, titleShowTime);
    }, avatarShowTime);
  });

  onDestroy(() => {
    yippieAudio.pause();
    drumAudio.pause();
    if (clearTimeout) {
      clearTimeout(currentTimeout);
    }
  });
</script>

<div class="flex container h-[95vh] items-center justify-center">
  {#if !offscreenDone}
    <p>Loading...</p>
  {/if}

  {#if offscreenDone && !titleHide}
    <p transition:fade={{ duration: 200 }} class="text-4xl">{topChatterRevealTitle}</p>
  {/if}

  <canvas
    bind:this={canvas}
    in:fade={{ duration: 200, delay: 1000 }}
    class="absolute min-w-[60%] md:min-w-[20%] {offscreenDone ? '' : 'invisible'}"
  ></canvas>
  <canvas
    bind:this={confettiCanvas}
    in:fade={{ duration: 200, delay: 200 }}
    class="absolute w-full h-[90%] {offscreenDone ? '' : 'invisible'}"
  ></canvas>
  <audio bind:this={yippieAudio} autoplay={false} muted={false} src="./audio/yippie.mp3"></audio>
  <audio bind:this={drumAudio} autoplay={false} muted={false} src="./audio/drum.mp3"></audio>
</div>
