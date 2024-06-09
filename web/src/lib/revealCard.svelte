<script lang="ts">
  import { onMount, tick } from 'svelte';
  import canvasConfetti from 'canvas-confetti';
  let canvas: HTMLCanvasElement;
  let confettiCanvas: HTMLCanvasElement;
  let offscreen: OffscreenCanvas;

  let offscreenDone = false;

  // TODO: These are placeholders
  const AVATAR_URL =
    'https://64.media.tumblr.com/d55402ac7df5b658290fca647b1d4300/fa59626f4dee859b-14/s1280x1920/b86af79b7490871b668274dde83532bfc81d434b.jpg';
  const AVATAR_WIDTH = 200;
  const AVATAR_HEIGHT = 200;
  const NAME = 'test';

  function drawConfetti(canvas: HTMLCanvasElement) {
    canvasConfetti.create(canvas, {
      resize: true
    })({
      particleCount: 100,
      spread: 160,
      origin: { x: 0.5, y: 0.5 }
    });
  }

  function drawOffscreen(parent: CanvasRenderingContext2D): Promise<void> {
    const ctx = offscreen.getContext('2d');
    if (!ctx) {
      throw new Error('2d context not supported');
    }

    const img = new Image();
    img.src = AVATAR_URL;
    return new Promise((resolve) => {
      img.onload = () => {
        ctx.drawImage(img, 0, 0, img.width, img.height, 0, 0, AVATAR_WIDTH, AVATAR_HEIGHT);
        ctx.font = '30px Arial';
        ctx.fillText(NAME, 10, 50);
        parent.drawImage(offscreen, 0, 0);
        offscreenDone = true;
        resolve();
      };
    });
  }

  onMount(async () => {
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      throw new Error('2d context not supported');
    }

    offscreen = new OffscreenCanvas(200, 200);
    await drawOffscreen(ctx);
    drawConfetti(confettiCanvas);
  });
</script>

<div class="flex container h-full items-center justify-center">
  <canvas bind:this={canvas} class="absolute" style="width: 200px; height: 200px"></canvas>
  <canvas bind:this={confettiCanvas} class="absolute" style="width: 400px; height: 400px"></canvas>
</div>
