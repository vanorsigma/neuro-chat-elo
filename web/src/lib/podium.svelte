<script lang="ts">
  import { onMount } from 'svelte';
  import type { User } from './user';
  import { sanitizeString } from '$lib';

  const avatarToPodiumPadding = 20;

  let canvasElement: HTMLCanvasElement;
  let canvasWidth: number = 400;
  let canvasHeight: number = 400;
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
    let secondPlaceRatio = secondPlace.elo / firstPlace.elo;
    let thirdPlaceRatio = thirdPlace.elo / firstPlace.elo;
    return [podiumHeight, podiumHeight * secondPlaceRatio, podiumHeight * thirdPlaceRatio];
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
    const context = canvasElement.getContext('2d');
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
</script>

<canvas bind:this={canvasElement} width={canvasWidth} height={canvasHeight}></canvas>
