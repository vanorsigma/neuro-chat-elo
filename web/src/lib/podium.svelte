<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { User } from './user';
  import { sanitizeString } from '$lib';

  let canvasElement: HTMLCanvasElement;
  let canvasWidth: number = 800;
  let canvasHeight: number = 600;

  export let firstPlace: User;
  export let secondPlace: User;
  export let thirdPlace: User;

  const avatarSize: number = 128;
  const nameFontSize = 20;
  const nameFont = `${nameFontSize}px arial bold`;

  const eloFontSize = 10;
  const eloFont = `${eloFontSize}px arial`;

  const colors = ['#C0C0C0', '#FFD700', '#CD7F32'];

  /**
   * Creates the user hero avatar thingy on top of the podium
   *
   * @argument parentContext {CanvasRenderingContext2D} The parent drawing context
   * @argument elo {number} The user's elo
   * @argument avatar_url {string} The URL to the profile pic of the user
   * @argument podium_x {number} The podium's X offset
   * @argument podium_y {number} The podium's y offset
   * @argument podium_w {number} The podium's width
   */
  function createUserCanvas(
    parentContext: CanvasRenderingContext2D,
    name: string,
    elo: number,
    avatar_url: string,
    podium_x: number,
    podium_y: number,
    podium_w: number
  ) {
    const avatarXOffset = (podium_w - avatarSize) / 2;
    const avatarYOffset = nameFontSize;

    const width = podium_w;
    const height = avatarSize + nameFontSize + eloFontSize + 10;

    const element = new OffscreenCanvas(width, height);
    const context = element.getContext('2d');

    if (context === undefined || context === null) {
      console.error('Offscreen Canvas is null');
      return;
    }

    context.font = nameFont;
    context.textAlign = 'center';
    context.fillText(
      sanitizeString(name),
      avatarXOffset + avatarSize / 2,
      avatarSize + nameFontSize
    );

    context.font = eloFont;
    context.textAlign = 'center';
    context.fillText(
      `Elo: ${elo}`,
      avatarXOffset + avatarSize / 2,
      avatarSize + nameFontSize + 2 + eloFontSize
    );

    const image = new Image(avatarSize, avatarSize);
    image.addEventListener('load', () => {
      context.drawImage(image, avatarXOffset, 0, avatarSize, avatarSize);
      parentContext.drawImage(element, podium_x, podium_y - avatarYOffset);
    });
    image.src = avatar_url;
  }

  function widthFromString(str: string) {
    return Math.max(100, Math.min(200, str.length * nameFontSize));
  }

  /**
   * Draws a podium based on the parameters given.
   *
   * @argument context The drawing context
   * @argument color Color of the podium
   * @argument user A {User} object that contains things
   * @argument xOffset The xOffset in the canvas
   * @argument yOffset The yOffset to draw the podium (it's the base of the entire podium)
   * @argument maxElo The maximum elo in the system (for relative scaling)
   * @argument minElo The minimum elo in the system (also for relative scaling)
   * @returns [number, number] The new xOffset to use for the next podium
   */
  function drawPodium(
    context: CanvasRenderingContext2D,
    color: string,
    user: User,
    xOffset: number,
    yOffset: number,
    maxElo: number,
    minElo: number
  ) {
    const width = widthFromString(user.name);
    const height = 50 + ((user.elo - minElo) / (maxElo - minElo)) * 100;

    context.fillStyle = color;
    context.fillRect(
      xOffset,
      yOffset - height + avatarSize + nameFontSize + eloFontSize + 20,
      width,
      height
    );

    context.fillStyle = 'black';
    context.textAlign = 'center';

    createUserCanvas(
      context,
      user.name,
      user.elo,
      user.avatar,
      xOffset,
      yOffset - height + 20,
      width
    );

    return xOffset + width;
  }

  onMount(async () => {
    const context = canvasElement.getContext('2d');
    const maxElo = Math.max(firstPlace.elo, secondPlace.elo, thirdPlace.elo);
    const minElo = Math.min(firstPlace.elo, secondPlace.elo, thirdPlace.elo);

    if (!context) {
      console.error('Cannot get context of Podium');
      return;
    }

    canvasWidth = [firstPlace, secondPlace, thirdPlace].reduce(
      (accum, user) => accum + widthFromString(user.name),
      0
    );
    canvasHeight = 150 + avatarSize + nameFontSize + eloFontSize + 20;
    await tick(); // wait for canvas width to update

    let currentX = drawPodium(context, colors[0], secondPlace, 0, 150, maxElo, minElo);
    currentX = drawPodium(context, colors[1], firstPlace, currentX, 150, maxElo, minElo);
    drawPodium(context, colors[2], thirdPlace, currentX, 150, maxElo, minElo);
  });
</script>

<canvas bind:this={canvasElement} width={canvasWidth} height={canvasHeight}></canvas>
