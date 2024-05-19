<script lang="ts">
  import { onMount, tick } from 'svelte';

  let canvasElement: HTMLCanvasElement;
  let canvasWidth: number = 800;
  let canvasHeight: number = 600;

  const avatarSize: number = 128;
  const nameFontSize = 20;
  const nameFont = `${nameFontSize}px arial bold`;

  const eloFontSize = 10;
  const eloFont = `${eloFontSize}px arial`;

  function createUserCanvas(
    parentContext: CanvasRenderingContext2D,
    name: string,
    elo: number,
    avatar_url: string,
    podium_x: number,
    podium_y: number,
    podium_w: number,
    podium_h: number
  ) {
    // const avatarSize = podium_w - 5;
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
    context.fillText(name, avatarXOffset + avatarSize / 2, avatarSize + nameFontSize);

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

  onMount(async () => {
    const podiums = [
      {
        y: 100,
        width: Math.max(100, Math.min(200, 'Someone else'.length * nameFontSize)),
        height: 100,
        color: '#C0C0C0',
        user: {
          name: 'Someone else',
          elo: 200,
          avatar:
            'https://64.media.tumblr.com/80467bc9c6f4b85ae470c63312c6b73f/fa59626f4dee859b-49/s540x810/1a00796571563255ed587431644ec78bfcca7db9.jpg'
        }
      },
      {
        // Top Chatter
        y: 50,
        width: Math.max(100, Math.min(200, 'Someone'.length * nameFontSize)), // TODO: Need to make this dynamic based on username
        height: 150,
        color: '#FFD700',
        user: {
          name: 'Someone',
          elo: 300,
          avatar: 'https://i.pinimg.com/474x/b8/10/b7/b810b717e748149f5b8a39daabff88a4.jpg'
        }
      },
      {
        y: 125,
        width: Math.max(100, Math.min(200, 'not important'.length * nameFontSize)),
        height: 75,
        color: '#CD7F32',
        user: {
          name: 'not important',
          elo: 300,
          avatar: 'https://i.pinimg.com/736x/9f/92/d6/9f92d6377ddf5280f71ff345987f2df7.jpg'
        }
      }
    ];
    const context = canvasElement.getContext('2d');
    canvasWidth = podiums.reduce((accum, podium) => accum + podium.width, 0);
    canvasHeight =
      podiums.reduce((accum, podium) => Math.max(accum, podium.height), 0) +
      avatarSize +
      nameFontSize +
      eloFontSize +
      20;
    await tick(); // wait for canvas width to update
    let currentX = 0;

    podiums.forEach((podium) => {
      // derive width from username
      context.fillStyle = podium.color;
      context.fillRect(currentX, podium.y + avatarSize + nameFontSize, podium.width, podium.height);

      context.fillStyle = 'black';
      context.textAlign = 'center';

      createUserCanvas(
        context,
        podium.user.name,
        podium.user.elo,
        podium.user.avatar,
        currentX,
        podium.y,
        podium.width,
        podium.height
      );
      currentX += podium.width;
    });
  });
</script>

<canvas bind:this={canvasElement} width={canvasWidth} height={canvasHeight}></canvas>
