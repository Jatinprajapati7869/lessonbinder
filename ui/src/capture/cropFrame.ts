export type PixelRect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export async function cropPngFromDataUrl(
  dataUrl: string,
  rect: PixelRect
): Promise<Uint8Array> {
  const image = await loadImage(dataUrl);
  const width = Math.max(1, Math.round(rect.width));
  const height = Math.max(1, Math.round(rect.height));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("Could not prepare frame crop.");
  }
  ctx.drawImage(image, rect.x, rect.y, width, height, 0, 0, width, height);
  const blob = await new Promise<Blob | null>((resolve) => {
    canvas.toBlob((value) => resolve(value), "image/png");
  });
  if (!blob) {
    throw new Error("Could not export frame crop.");
  }
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}

function loadImage(dataUrl: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = () => reject(new Error("Could not load frame image."));
    img.src = dataUrl;
  });
}
