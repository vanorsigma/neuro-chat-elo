<script lang="ts">
  import { onMount } from 'svelte';
  import type { User } from './user';
  import { sanitizeString } from '$lib';

  const avatarToPodiumPadding = 20;

  let canvasElement: HTMLCanvasElement;
  let canvasWidth: number = 400;
  let canvasHeight: number = 400;

  const minimumHeight = 110;
  $: maximumHeight = podiumHeight;

  $: podiumWidth = canvasWidth / 3;
  $: podiumHeight = canvasHeight - canvasToPodiumOffset;

  export let scaleToX: number;

  $: canvasWidth = scaleToX;
  $: canvasHeight = scaleToX;
  $: canvasToPodiumOffset = scaleToX * 0.5;

  export let firstPlace: User;
  export let secondPlace: User;
  export let thirdPlace: User;

  const colors = ['#C0C0C0', '#FFD700', '#CD7F32'];

  function calculateRelativeHeights() {
    // NOTE: credit ByronOf39
    let elements = [firstPlace.elo, secondPlace.elo, thirdPlace.elo];

    // If all elements are the same, return a default scaled array
    if (elements[2] === elements[0]) {
      const defaultValue = Math.round((minimumHeight + maximumHeight) / 2);
      return [defaultValue, defaultValue, defaultValue];
    }

    const maxElementAbove = elements[2] - elements[0];
    const deltaHeight = maximumHeight - minimumHeight;

    // Scale elements
    elements = elements.map((element, index) => {
      const above = element - elements[2];
      let fraction = above / maxElementAbove;
      return index === 0 ? maximumHeight : fraction * deltaHeight + minimumHeight;
    });

    const minDiffHeight = deltaHeight * 0.16666666;
    let last = maximumHeight + minDiffHeight;

    // Clamp to max
    for (let i = 0; i < elements.length; i++) {
      if (elements[i] === last) {
        elements[i - 1] -= minDiffHeight;
        elements[i] -= minDiffHeight;
        continue;
      }

      while (minDiffHeight + elements[i] > last) {
        elements[i] -= minDiffHeight;
      }
      last = elements[i];
    }
    last = minimumHeight - minDiffHeight;

    // Clamp to min
    for (let i = elements.length - 1; i >= 0; i--) {
      if (elements[i] === last) {
        continue;
      }

      while (elements[i] < last + minDiffHeight) {
        elements[i] += minDiffHeight;
      }
      last = elements[i];
    }
    return elements;
  }

  function drawPodiums(context: CanvasRenderingContext2D) {
    const relativeHeights = calculateRelativeHeights();
    context.fillStyle = colors[0];
    context.fillRect(
      0,
      podiumHeight - relativeHeights[1] + canvasToPodiumOffset,
      podiumWidth,
      relativeHeights[1]
    );

    context.fillStyle = colors[1];
    context.fillRect(
      podiumWidth,
      podiumHeight - relativeHeights[0] + canvasToPodiumOffset,
      podiumWidth,
      podiumHeight
    );

    context.fillStyle = colors[2];
    context.fillRect(
      podiumWidth * 2,
      podiumHeight - relativeHeights[2] + canvasToPodiumOffset,
      podiumWidth,
      podiumHeight
    );
  }

  function drawHero(
    user: User,
    position: number,
    actualPodiumHeight: number,
    context: CanvasRenderingContext2D
  ) {
    const image = new Image(128, 128);
    const avatarSize = podiumWidth - avatarToPodiumPadding;
    const avatarXOffset = position * podiumWidth + avatarToPodiumPadding / 2;
    const avatarYOffset = canvasHeight - actualPodiumHeight - avatarSize - avatarToPodiumPadding;
    const nameSize = user.name.length;
    image.addEventListener('load', () => {
      context?.drawImage(image, avatarXOffset, avatarYOffset, avatarSize, avatarSize);

      context.font = `${(1 / nameSize) * 175}px arial`;
      context.fillStyle = 'black';
      context.textAlign = 'center';
      context?.fillText(
        sanitizeString(user.name),
        avatarXOffset + avatarSize / 2,
        avatarYOffset - 10
      );

      context.font = `0.7em arial`;
      context.fillStyle = 'black';
      context.textAlign = 'center';
      context?.fillText(
        `${user.elo.toFixed(2)}`,
        avatarXOffset + avatarSize / 2,
        avatarYOffset + avatarSize + 12.5
      );
    });
    image.src = user.avatar;
  }

  onMount(() => {
    const context = convertHiDPICanvas(canvasElement, canvasWidth, canvasHeight);
    if (!context) {
      console.error('Cannot get context to draw podium');
      return;
    }
    const relativeHeights = calculateRelativeHeights();
    drawPodiums(context);
    drawHero(secondPlace, 0, relativeHeights[1], context);
    drawHero(firstPlace, 1, relativeHeights[0], context);
    drawHero(thirdPlace, 2, relativeHeights[2], context);
  });

  function convertHiDPICanvas(canvas: HTMLCanvasElement, width: number, height: number) {
    const ratio = Math.ceil(window.devicePixelRatio);
    canvas.width = width * ratio;
    canvas.height = height * ratio;
    canvas.style.width = `${width}px`;
    canvas.style.height = `${height}px`;
    const context = canvas.getContext('2d');
    context?.setTransform(ratio, 0, 0, ratio, 0, 0);
    return context;
  }
</script>

<canvas bind:this={canvasElement} width={canvasWidth} height={canvasHeight}></canvas>
