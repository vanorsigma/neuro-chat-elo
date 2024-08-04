<script>
  const fontFamily = 'Comic Sans MS';
  export let username = 'alternate person';
  export let eloScore = 1401.0;
  export let avatarUrl =
    'https://atlas-content-cdn.pixelsquid.com/stock-images/egg-z0nrae1-600.jpg';
  export let iconUrl =
    'https://i.pinimg.com/originals/c1/8a/8e/c18a8e6298ce417e595b4ddec7bc486b.png';
</script>

<svg
  class="w-full md:w-auto my-[-1vh] md:my-0"
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  version="1.1"
  width="506"
  height="67"
  viewBox="-0.5 -0.5 506 67"
>
  <defs>
    <filter id="dropShadow">
      <feGaussianBlur in="SourceAlpha" stdDeviation="1.7" result="blur" />
      <feOffset in="blur" dx="3" dy="3" result="offsetBlur" />
      <feFlood flood-color="#3D4574" flood-opacity="0.4" result="offsetColor" />
      <feComposite in="offsetColor" in2="offsetBlur" operator="in" result="offsetBlur" />
      <feBlend in="SourceGraphic" in2="offsetBlur" />
    </filter>
  </defs>
  <g filter="url(#dropShadow)">
    <g data-cell-id="0">
      <g data-cell-id="1">
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-1">
          <g style="filter: drop-shadow(rgba(0, 0, 0, 0.25) 2px 3px 2px);">
            <path
              d="M 244 54 L 14 54 Q 4 54 5.96 44.19 L 12.04 13.81 Q 14 4 24 4 L 484 4 Q 494 4 492.04 13.81 L 485.96 44.19 Q 484 54 474 54 Z"
              fill="url(#mx-gradient-ffd700-1-ffb570-1-e-0)"
              stroke="none"
              pointer-events="all"
            />
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-2">
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
                <div
                  xmlns="http://www.w3.org/1999/xhtml"
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 29px; margin-left: 35px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 12px; font-family: &quot;Helvetica&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      <font style="font-size: 30px; color: black">#1</font>
                    </div>
                  </div>
                </div>
                <text
                  x="64"
                  y="38"
                  fill="rgb(0, 0, 0)"
                  font-family={fontFamily}
                  font-size="18px"
                  text-anchor="middle">#1</text
                >
              </switch>
            </g>
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-3">
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
                  x="250"
                  y="35"
                  fill="rgb(0, 0, 0)"
                  font-family={fontFamily}
                  font-size="{(1 / Math.max(14, username.length)) * 250}px"
                  text-anchor="middle">{username}</text
                >
              </switch>
            </g>
          </g>
        </g>
        <g data-cell-id="TQTXx6ygs8IL0Gh_ZsiX-6">
          <g>
            <rect
              x="404"
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
              <text
                x="434"
                y="33"
                fill="rgb(0, 0, 0)"
                font-family={fontFamily}
                font-size="12px"
                text-anchor="middle"
                font-weight="bold">{eloScore}</text
              >
            </g>
          </g>
        </g>
        <g data-cell-id="gtfplMh8BVE9ldx_d5it-3">
          <g>
            <image href={avatarUrl} x="114" y="6.5" width="45" height="45" />
          </g>
        </g>
        <g data-cell-id="gtfplMh8BVE9ldx_d5it-6">
          <g> <image href={iconUrl} x="330" y="14" width="32.5" height="32.5"> </image></g>
        </g>
      </g>
    </g>
  </g>
</svg>
