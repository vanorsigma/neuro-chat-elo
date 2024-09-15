<script>
  import { loadImageAsBase64URL } from './rankutils';
  const fontFamily = 'Comic Sans MS';
  export let username = 'alternate person';
  export let eloScore = 1399.0;
  export let avatarUrl =
    'https://atlas-content-cdn.pixelsquid.com/stock-images/egg-z0nrae1-600.jpg';
  export let iconUrl =
    'https://i.pinimg.com/originals/c1/8a/8e/c18a8e6298ce417e595b4ddec7bc486b.png';

  var avatarUrlResolved = '';
  var iconUrlResolved = '';

  $: {
    const promise = async () => {
      avatarUrlResolved = await loadImageAsBase64URL(avatarUrl);
      iconUrlResolved = await loadImageAsBase64URL(iconUrl);
    };
    promise();
  }
</script>

<svg
  class="w-[90%] h-auto"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  version="1.1"
  width="456"
  height="67"
  viewBox="-0.5 -0.5 456 67"
>
  <g>
    <g data-cell-id="0">
      <g data-cell-id="1">
        <g data-cell-id="G50rVx7S7XyaNfVeGgGd-32">
          <g style="filter: drop-shadow(rgba(0, 0, 0, 0.25) 2px 3px 2px);">
            <path
              d="M 219 54 L 14 54 Q 4 54 5.96 44.19 L 12.04 13.81 Q 14 4 24 4 L 434 4 Q 444 4 442.04 13.81 L 435.96 44.19 Q 434 54 424 54 Z"
              fill="url(#mx-gradient-ff33ff-1-660066-1-e-0)"
              stroke="none"
              pointer-events="all"
            />
          </g>
        </g>
        <g data-cell-id="G50rVx7S7XyaNfVeGgGd-9">
          <g style="filter: drop-shadow(rgba(0, 0, 0, 0.25) 2px 3px 2px);">
            <path
              d="M 219 49 L 22 49 Q 12 49 14.43 39.3 L 19.57 18.7 Q 22 9 32 9 L 426 9 Q 436 9 433.57 18.7 L 428.43 39.3 Q 426 49 416 49 Z"
              fill="url(#mx-gradient-613e00-1-ffb570-1-e-0)"
              stroke="none"
              pointer-events="all"
            />
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-14">
          <g>
            <rect
              x="34"
              y="14"
              width="60"
              height="30"
              fill="none"
              stroke="none"
              pointer-events="all"
            />
          </g>
          <g>
            <g transform="translate(-0.5 -0.5)">
              <switch>
                <text
                  x="64"
                  y="33"
                  fill="rgb(255, 255, 255)"
                  font-family={fontFamily}
                  font-size="18px"
                  text-anchor="middle">#3</text
                >
              </switch>
            </g>
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-15">
          <g>
            <rect
              x="164"
              y="14"
              width="190"
              height="30"
              fill="none"
              stroke="none"
              pointer-events="all"
            />
          </g>
          <g>
            <g transform="translate(-0.5 -0.5)">
              <switch>
                <text
                  x="259"
                  y="33"
                  fill="rgb(0, 0, 0)"
                  font-family={fontFamily}
                  font-size="{(1 / Math.max(14, username.length)) * 250}px"
                  text-anchor="middle">{username}</text
                >
              </switch>
            </g>
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-18">
          <g>
            <rect
              x="374"
              y="14"
              width="60"
              height="30"
              fill="none"
              stroke="none"
              pointer-events="all"
            />
          </g>
          <g>
            <g transform="translate(-0.5 -0.5)">
              <switch>
                <text
                  x="404"
                  y="33"
                  fill="rgb(0, 0, 0)"
                  font-family={fontFamily}
                  font-size="12px"
                  text-anchor="middle"
                  font-weight="bold">{eloScore.toFixed(0)}</text
                >
              </switch>
            </g>
          </g>
        </g>
        <g data-cell-id="gtfplMh8BVE9ldx_d5it-5">
          <g>
            <image href={avatarUrlResolved} x="114" y="6.5" width="45" height="45" />
          </g>
        </g>
        <g data-cell-id="gtfplMh8BVE9ldx_d5it-8">
          <g>
            <image
              href={iconUrlResolved}
              x="330"
              y="12.75"
              width="32.5"
              height="32.5"
              crossorigin=""
            >
            </image>
          </g>
        </g>
        <g data-cell-id="gtfplMh8BVE9ldx_d5it-9" />
      </g>
    </g>
  </g>
</svg>
