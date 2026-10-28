export async function captureVideoFrameJpeg(
  video: HTMLVideoElement
): Promise<Uint8Array> {
  const canvas = document.createElement("canvas");
  canvas.width = video.videoWidth;
  canvas.height = video.videoHeight;
  const ctx = canvas.getContext("2d");
  if (!ctx || canvas.width === 0 || canvas.height === 0) {
    throw new Error("No video frame to capture.");
  }
  ctx.drawImage(video, 0, 0);
  const dataUrl = canvas.toDataURL("image/jpeg", 0.92);
  const base64 = dataUrl.split(",", 2)[1];
  if (!base64) {
    throw new Error("Could not encode frame.");
  }
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}
