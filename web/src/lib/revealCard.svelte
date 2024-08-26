<script lang="ts">
  import { loadImageAsBase64URL } from './rankitems/rankutils';

  export let username = '';
  export let avatarUrl = '';
  export let title = '';
  export let winner = false;

  var avatarUrlResolved = '';

  $: {
    const promise = async () => {
      avatarUrlResolved = await loadImageAsBase64URL(avatarUrl);
    };
    promise();
  }
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  version="1.1"
  width="20vmax"
  viewBox="-0.5 -0.5 171 226"
>
  <clipPath id="circleClip">
    <circle cx="85" cy="113" r="60" />
  </clipPath>

  <g>
    <g data-cell-id="0">
      <g data-cell-id="1">
        <g data-cell-id="WxxLEZ8-NIePb2jUWeOi-9">
          <g>
            <rect
              x="0"
              y="0"
              width="170"
              height="225"
              rx="25.5"
              ry="25.5"
              fill={winner ? 'url(#mx-gradient-ffd700-1-d4c11e-1-s-0)' : '#f7dedf'}
              pointer-events="all"
            />
          </g>
        </g>
        <g data-cell-id="WxxLEZ8-NIePb2jUWeOi-2">
          <g>
            <rect
              x="42.25"
              y="15"
              width="85"
              height="30"
              fill="none"
              stroke="none"
              pointer-events="all"
            />
          </g>
          <g>
            <g transform="translate(-0.5 -0.5)">
              <foreignObject
                style="overflow: visible; text-align: left;"
                pointer-events="none"
                width="100%"
                height="100%"
                requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility"
              >
                <div
                  xmlns="http://www.w3.org/1999/xhtml"
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 83px; height: 1px; padding-top: 30px; margin-left: 43px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style={'display: inline-block; font-family: &quot;Helvetica&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;' +
                        ` font-size: ${(1 / username.length) * 200}px;`}
                    >
                      {username}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="WxxLEZ8-NIePb2jUWeOi-3">
          <g>
            <rect
              x="25"
              y="175"
              width="117.5"
              height="30"
              fill="none"
              stroke="none"
              pointer-events="all"
            />
          </g>
          <g>
            <g transform="translate(-0.5 -0.5)">
              <foreignObject
                style="overflow: visible; text-align: left;"
                pointer-events="none"
                width="100%"
                height="100%"
                requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility"
              >
                <div
                  xmlns="http://www.w3.org/1999/xhtml"
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 116px; height: 1px; padding-top: 190px; margin-left: 26px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 15px; font-family: &quot;Helvetica&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {title}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="WxxLEZ8-NIePb2jUWeOi-8">
          <image
            xlink:href={avatarUrlResolved}
            x="25"
            y="52.5"
            width="120"
            height="120"
            clip-path="url(#circleClip)"
          />
        </g>
      </g>
    </g>
  </g>
</svg>
