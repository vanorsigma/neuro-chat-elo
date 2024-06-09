<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { User } from './user';
  import { sanitizeString } from '$lib';

  let canvasElement: HTMLCanvasElement;
  let canvasWidth: number = 800;
  let canvasHeight: number = 600;

  export let scaleToX: number = 400;

  const aspectRatio = canvasWidth / canvasHeight;

  export let firstPlace: User;
  export let secondPlace: User;
  export let thirdPlace: User;

  const avatarSize: number = 128;
  const nameFontSize = 20;
  const nameFont = `${nameFontSize}px arial bold`;

  const eloFontSize = 15;
  const eloFont = `${eloFontSize}px arial`;

  const colors = ['#C0C0C0', '#FFD700', '#CD7F32'];

  /**
   * Creates the user hero avatar thingy on top of the podium
   *
   * @argument parentCanvas {OffscreenCanvas} The parent drawing canvas
   * @argument elo {number} The user's elo
   * @argument avatar_url {string} The URL to the profile pic of the user
   * @argument podium_x {number} The podium's X offset
   * @argument podium_y {number} The podium's y offset
   * @argument podium_w {number} The podium's width
   */
  function createUserCanvas(
    parentCanvas: OffscreenCanvas,
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
      `Elo: ${elo.toFixed(2)}`,
      avatarXOffset + avatarSize / 2,
      avatarSize + nameFontSize + 2 + eloFontSize
    );

    const image = new Image(avatarSize, avatarSize);
    image.addEventListener('load', () => {
      const parentContext = parentCanvas.getContext('2d');
      if (!parentContext) {
        console.error('Parent context is null');
        return;
      }

      context.drawImage(image, avatarXOffset, 0, avatarSize, avatarSize);
      parentContext.drawImage(element, podium_x, podium_y - avatarYOffset);

      const actualCanvas = canvasElement.getContext('2d');
      actualCanvas?.drawImage(
        parentCanvas,
        0,
        0,
        canvasWidth,
        canvasHeight,
        0,
        0,
        scaleToX,
        scaleToX / aspectRatio
      );
    });
    image.src = avatar_url;
  }

  function widthFromString(str: string) {
    return Math.max(128 + 5, Math.min(200, str.length * nameFontSize));
  }

  /**
   * Draws a podium based on the parameters given.
   *
   * @argument canvs The drawing canvas
   * @argument color Color of the podium
   * @argument user A {User} object that contains things
   * @argument xOffset The xOffset in the canvas
   * @argument yOffset The yOffset to draw the podium (it's the base of the entire podium)
   * @argument maxElo The maximum elo in the system (for relative scaling)
   * @argument minElo The minimum elo in the system (also for relative scaling)
   * @returns [number, number] The new xOffset to use for the next podium
   */
  function drawPodium(
    canvas: OffscreenCanvas,
    color: string,
    user: User,
    xOffset: number,
    yOffset: number,
    maxElo: number,
    minElo: number
  ) {
    const width = widthFromString(user.name);
    const height = 50 + ((user.elo - minElo) / (maxElo - minElo)) * 100;
    const context = canvas.getContext('2d');

    if (context === null) {
      console.error('Drawing context is null, cannot continue');
      return;
    }

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
      canvas,
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
    const drawingElement = new OffscreenCanvas(canvasWidth, canvasHeight);
    const context = drawingElement.getContext('2d');

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

    let currentX = drawPodium(drawingElement, colors[0], secondPlace, 0, 150, maxElo, minElo);
    currentX = drawPodium(drawingElement, colors[1], firstPlace, currentX, 150, maxElo, minElo);
    drawPodium(drawingElement, colors[2], thirdPlace, currentX, 150, maxElo, minElo);

    const actualCanvas = canvasElement.getContext('2d');
    actualCanvas?.drawImage(
      drawingElement,
      0,
      0,
      canvasWidth,
      canvasHeight,
      0,
      0,
      scaleToX,
      scaleToX / aspectRatio
    );
  });
</script>

<canvas bind:this={canvasElement} width={scaleToX} height={scaleToX / aspectRatio}></canvas>
