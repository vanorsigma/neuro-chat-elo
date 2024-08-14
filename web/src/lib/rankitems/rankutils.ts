/**
 * This function preloads images, (also tries to bypass hotlinking protection)
 * It returns a base64 encoded URL that represents the loaded image
 */
export function loadImageAsBase64URL(src: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = '';
    img.referrerPolicy = 'no-referrer';

    img.onload = () => {
      const canvas = document.createElement('canvas');
      canvas.width = img.width;
      canvas.height = img.height;
      const ctx = canvas.getContext('2d');
      if (ctx) {
        ctx.drawImage(img, 0, 0);
        const dataURL = canvas.toDataURL('image/png');
        resolve(dataURL);
      } else {
        reject(new Error('cannot create in-memory canvas'));
      }
    };

    img.onerror = (error) => {
      reject(new Error(`Failed to load image: ${error}`));
    };

    img.src = src;
  })
}
