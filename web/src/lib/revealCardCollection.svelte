<script lang="ts">
  import { onMount } from 'svelte';
  import RevealCardBasic from './revealCardBasic.svelte';
  import canvasConfetti from 'canvas-confetti';
  import type { RevealMetadata } from './revealMetadata';
  import { fade, fly } from 'svelte/transition';

  let collection: HTMLDivElement;
  let winnerCard: HTMLDivElement;
  let confettiCanvas: HTMLCanvasElement;
  let offscreenDone = false;
  let finishedLoadingCount = 0;
  export let revealMetadatas: RevealMetadata[] = [];
  // NOTE: this variable should not be used directly. use the cards element below
  let _cards: HTMLDivElement[] = [];
  $: other_cards = _cards.filter((card) => card);

  function drawConfetti(canvas: HTMLCanvasElement) {
    canvasConfetti.create(canvas, {
      resize: true
    })({
      particleCount: 300,
      spread: 160,
      origin: { x: 0.5, y: 0.5 }
    });
  }

  function getRevealMetadataOrdered() {
    let copied = revealMetadatas;
    let length = copied.length;
    let first = copied[0];
    let middle = copied[Math.round(length / 2)];

    copied[0] = middle;
    copied[Math.round(length / 2)] = first;
    return copied;
  }

  function getHeightOffset(metadata: RevealMetadata) {
    if (metadata.leaderboardName === 'Overall') {
      return 0;
    }
    return 100;
  }

  function onFinishedLoading() {
    finishedLoadingCount += 1;
    if (finishedLoadingCount >= revealMetadataOrdered.length) {
      offscreenDone = true;
    }
  }

  function getAnimationOffsetX(element: HTMLDivElement) {
    return winnerCard.getBoundingClientRect().left - element.getBoundingClientRect().left;
  }

  function _performAppearAnimationForOtherCards() {
    const slideFromAndAppear = (element: HTMLDivElement) => {
      let originalOffset = getAnimationOffsetX(element);
      element.style.transform = `translate(${originalOffset}px, 0px)`;

      // give it one javascript queue cycle to do the animation
      setTimeout(() => {
        element.style.transform = `translate(0px, 0px)`;
        element.style.opacity = '1.0';
        setTimeout(() => {
          element.style.transition = 'transform 2s ease, opacity 1s ease';
        });
      }, 1000);
    };

    _cards.forEach((card) => {
      if (card) {
        slideFromAndAppear(card);
      }
    });
  }

  // CSS animations don't play properly
  function performAppearAnimation() {
    winnerCard.style.opacity = '0.0';
    _cards.forEach((card) => {
      if (card) {
        card.style.opacity = '0.0';
      }
    });

    setTimeout(() => {
      winnerCard.style.opacity = '1.0';
      setTimeout(() => {
        winnerCard.style.transition = 'opacity 1s ease';
      });
      setTimeout(() => {
        _performAppearAnimationForOtherCards();
        drawConfetti(confettiCanvas);
      }, 1000);
    }, 1000);
  }

  $: revealMetadataOrdered = getRevealMetadataOrdered();

  onMount(() => {
    winnerCard.scrollIntoView({
      behavior: 'auto',
      block: 'center',
      inline: 'center'
    });
    performAppearAnimation();
  });
</script>

{#if !offscreenDone}
  <p>Loading...</p>
{/if}

<div
  bind:this={collection}
  class="absolute flex h-full overflow-x-scroll w-full items-center gap-5 {offscreenDone
    ? ''
    : 'invisible'}"
>
  {#each revealMetadataOrdered as metadata, index}
    {#if metadata.leaderboardName == 'Overall'}
      <div bind:this={winnerCard}>
        <RevealCardBasic
          avatarUrl={metadata.avatarUrl}
          avatarName={metadata.avatarName}
          heightOffset={getHeightOffset(metadata)}
          title={metadata.leaderboardName}
          onFinishedLoading={() => onFinishedLoading()}
          winner={true}
        ></RevealCardBasic>
      </div>
    {:else}
      <div bind:this={_cards[index]}>
        <RevealCardBasic
          avatarUrl={metadata.avatarUrl}
          avatarName={metadata.avatarName}
          heightOffset={getHeightOffset(metadata)}
          title={metadata.leaderboardName}
          onFinishedLoading={() => onFinishedLoading()}
        ></RevealCardBasic>
      </div>
    {/if}
  {/each}
</div>

<canvas
  bind:this={confettiCanvas}
  in:fade={{ duration: 200, delay: 200 }}
  class="absolute w-full h-[90%] {offscreenDone ? '' : 'invisible'}"
></canvas>
