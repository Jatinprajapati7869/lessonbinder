export function isPictureInPictureSupported(): boolean {
  if (typeof document === "undefined") {
    return false;
  }
  if (!("pictureInPictureEnabled" in document)) {
    return false;
  }
  if (!document.pictureInPictureEnabled) {
    return false;
  }
  return (
    typeof HTMLVideoElement !== "undefined" &&
    "requestPictureInPicture" in HTMLVideoElement.prototype
  );
}

export async function toggleVideoPictureInPicture(
  video: HTMLVideoElement
): Promise<void> {
  if (document.pictureInPictureElement === video) {
    await document.exitPictureInPicture();
    return;
  }
  await video.requestPictureInPicture();
}
