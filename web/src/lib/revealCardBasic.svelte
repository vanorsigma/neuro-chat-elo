<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  let canvas: HTMLCanvasElement;
  let offscreen: OffscreenCanvas;

  let maxAvatarWidth = NaN;
  const textSize = 40;

  let offscreenDone = false;

  export let avatarUrl = '';
  export let avatarName = '';
  export let heightOffset = 0;
  export let title = '';
  export let winner = false;

  const defaultAvatarUrl =
    'https://static-cdn.jtvnw.net/user-default-pictures-uv/ebe4cd89-b4f4-4cd9-adac-2f30151b4209-profile_image-300x300.png';

  export let onFinishedLoading = () => {};

  function drawOffscreen(): Promise<OffscreenCanvasRenderingContext2D> {
    const ctx = offscreen.getContext('2d');
    if (!ctx) {
      throw new Error('2d context not supported');
    }

    return new Promise((resolve) => {
      let img_loaded = false;
      let crown_loaded = false;

      const img = new Image();
      img.src = avatarUrl;
      img.referrerPolicy = 'no-referrer';

      const laurel = new Image();
      laurel.src = 'https://upload.wikimedia.org/wikipedia/commons/8/80/Laurel.svg';

      const imgCancelIfDoneTimeout = setTimeout(() => {
        img.src = defaultAvatarUrl;
      }, 2000);

      img.onload = () => {
        img_loaded = true;
        clearTimeout(imgCancelIfDoneTimeout);
        both_images_loaded_callback();
      };

      laurel.onload = () => {
        crown_loaded = true;
        both_images_loaded_callback();
      };

      const both_images_loaded_callback = () => {
        if (!img_loaded || !crown_loaded) return;

        const offset = (canvas.width - maxAvatarWidth) * 0.5;

        ctx.save();
        ctx.beginPath();
        ctx.arc(maxAvatarWidth / 2, maxAvatarWidth / 2, maxAvatarWidth / 2, 0, Math.PI * 2, true);
        ctx.closePath();
        ctx.clip();
        ctx.drawImage(img, 0, 0, img.width, img.height, offset, 0, maxAvatarWidth, maxAvatarWidth);
        ctx.restore();

        if (winner) {
          ctx.fillStyle = 'blue';
          ctx.drawImage(
            laurel,
            0,
            0,
            laurel.width,
            laurel.height,
            offset,
            50,
            maxAvatarWidth,
            maxAvatarWidth
          );
        }
        ctx.textAlign = 'center';
        ctx.font = `${(1 / avatarName.length) * 270}px arial bold`;
        ctx.fillText(avatarName, canvas.width * 0.5, maxAvatarWidth + textSize / 2 + 40);

        ctx.font = `${(1 / title.length) * 150}px arial bold`;
        ctx.fillText(title, canvas.width * 0.5, maxAvatarWidth + textSize / 2 + 80);

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

    maxAvatarWidth = Math.min(canvas.clientWidth * 0.7, document.documentElement.clientWidth * 0.5);
    canvas.width = maxAvatarWidth;
    canvas.height = maxAvatarWidth + textSize * 3;

    offscreen = new OffscreenCanvas(canvas.width, canvas.height);
    await drawOffscreen();
    onFinishedLoading();

    ctx.drawImage(offscreen, 0, 0);
  });

  onDestroy(() => {});
</script>

<canvas
  bind:this={canvas}
  style="top: {heightOffset}px"
  class="relative {offscreenDone ? '' : 'invisible'}"
></canvas>
