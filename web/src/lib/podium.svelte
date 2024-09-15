<script lang="ts">
  import { onMount } from 'svelte';
  import type { User } from './user';
  import { loadImageAsBase64URL } from './rankitems/rankutils';
  import Podiumbar from './podiumbar.svelte';

  const minimumHeight = 110;
  const maximumHeight = 180;

  export let size: string;

  export let firstPlace: User;
  export let secondPlace: User;
  export let thirdPlace: User;

  const englishUsername = '[\\w!@#$%^&*(){};\':",.<>?/|\\[\\]_+\\- ]+';

  var firstAvatarUrlResolved = '';
  var secondAvatarUrlResolved = '';
  var thirdAvatarUrlResolved = '';

  var secondRelativeHeight = 1.0;
  var thirdRelativeHeight = 1.0;

  function isAlphaNumeric(str: string) {
    const matches = str.match(new RegExp(englishUsername));
    if (!matches) {
      return null;
    }
    return matches[0].length == str.length;
  }

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
    return elements.map((element) => element / maximumHeight);
  }

  function getFontSizeForText(text: string) {
    if (isAlphaNumeric(text)) {
      return `font-size: ${(1 / text.length) * 150}px; font-family: "Comic Sans MS";`;
    }

    return `font-size: ${(1 / text.length) * 100}px; font-family: "Comic Sans MS";`;
  }

  onMount(() => {
    // NOTE: This is fine only because there are only three positions (ever) on the podium
    const promise = async () => {
      firstAvatarUrlResolved = await loadImageAsBase64URL(firstPlace.avatar);
      secondAvatarUrlResolved = await loadImageAsBase64URL(secondPlace.avatar);
      thirdAvatarUrlResolved = await loadImageAsBase64URL(thirdPlace.avatar);
    };

    promise();

    const relativeHeights = calculateRelativeHeights();
    secondRelativeHeight = relativeHeights[1];
    thirdRelativeHeight = relativeHeights[2];
  });
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  version="1.1"
  width={size}
  height={size}
  viewBox="-0.5 -0.5 302 302"
>
  <defs />
  <g>
    <g data-cell-id="0">
      <g data-cell-id="1">
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-1">
          <g>
            <Podiumbar x={0} scaling={secondRelativeHeight} color="#c0c0c0" />
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-2">
          <g>
            <Podiumbar x={100} scaling={1.0} color="#ffd700" />
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-3">
          <g>
            <Podiumbar x={200} scaling={thirdRelativeHeight} color="#cd7f32" />
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-7">
          <g>
            <rect
              x="115"
              y="141"
              width="70"
              height="60"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 68px; height: 1px; padding-top: 171px; margin-left: 116px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 50px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; font-weight: bold; white-space: normal; overflow-wrap: normal;"
                    >
                      #1
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-8">
          <g>
            <rect
              x="15"
              y="181"
              width="70"
              height="60"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 68px; height: 1px; padding-top: {171 +
                    (1.0 - secondRelativeHeight) * maximumHeight}px; margin-left: 16px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 50px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; font-weight: bold; white-space: normal; overflow-wrap: normal;"
                    >
                      #2
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-9">
          <g>
            <rect
              x="215"
              y="211"
              width="70"
              height="60"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 68px; height: 1px; padding-top: {171 +
                    (1.0 - thirdRelativeHeight) * maximumHeight}px; margin-left: 216px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 50px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; font-weight: bold; white-space: normal; overflow-wrap: normal;"
                    >
                      #3
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-10">
          <g>
            <image
              href={secondAvatarUrlResolved}
              x="10"
              y={21 + (1.0 - secondRelativeHeight) * maximumHeight}
              height="80"
              width="80"
            >
            </image>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-11">
          <g>
            <image href={firstAvatarUrlResolved} x="110" y="21" height="80" width="80"> </image></g
          >
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-12">
          <g>
            <image
              href={thirdAvatarUrlResolved}
              x="210"
              y={21 + (1.0 - thirdRelativeHeight) * maximumHeight}
              height="80"
              width="80"
            >
            </image>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-13">
          <g>
            <rect
              x="20"
              y="141"
              width="60"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: {111 +
                    (1.0 - secondRelativeHeight) * maximumHeight}px; margin-left: 21px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 12px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {secondPlace.elo.toFixed(2)}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-14">
          <g>
            <rect
              x="120"
              y="101"
              width="60"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 111px; margin-left: 121px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 12px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {firstPlace.elo.toFixed(2)}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-15">
          <g>
            <rect
              x="22012"
              y="171"
              width="60"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: {111 +
                    (1.0 - thirdRelativeHeight) * maximumHeight}px; margin-left: 221px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; font-size: 12px; font-family: &quot;Comic Sans MS&quot;; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {thirdPlace.elo.toFixed(2)}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-16">
          <g>
            <rect
              x="5"
              y="41"
              width="90"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 88px; height: 1px; padding-top: {11 +
                    (1.0 - secondRelativeHeight) * maximumHeight}px; margin-left: 6px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; {getFontSizeForText(
                        secondPlace.name
                      )} color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {secondPlace.name}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-17">
          <g>
            <rect
              x="105"
              y="1"
              width="90"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 88px; height: 1px; padding-top: 11px; margin-left: 106px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; {getFontSizeForText(
                        firstPlace.name
                      )} color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {firstPlace.name}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
        <g data-cell-id="3iLCq0i9T9ulFOnHV9ir-18">
          <g>
            <rect
              x="205"
              y="71"
              width="90"
              height="20"
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
                  style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 88px; height: 1px; padding-top: {11 +
                    (1.0 - thirdRelativeHeight) * maximumHeight}px; margin-left: 206px;"
                >
                  <div
                    style="box-sizing: border-box; font-size: 0px; text-align: center;"
                    data-drawio-colors="color: rgb(0, 0, 0); "
                  >
                    <div
                      style="display: inline-block; {getFontSizeForText(
                        thirdPlace.name
                      )} color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"
                    >
                      {thirdPlace.name}
                    </div>
                  </div>
                </div>
              </foreignObject>
            </g>
          </g>
        </g>
      </g>
    </g>
  </g>
</svg>
