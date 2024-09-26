// place files you want to import through the `$lib` alias in this folder.


// From https://stackoverflow.com/a/12034334
const entityMap = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#39;',
  '/': '&#x2F;',
  '`': '&#x60;',
  '=': '&#x3D;'
};

/**
 * Sanitizes strings. In theory Twitch would have already cleaned the
 * string, but I'm doing this just in case
 *
 * @param str {string} The string to sanitize
 * @returns A sanitized string
 */
export function sanitizeString(str: string): string {
  return String(str).replace(/[&<>"'`=\/]/g, function (s) {
    return entityMap[s];
  });
}
