import { afterEach, describe, expect, it, vi } from "vitest";

import {
  isPictureInPictureSupported,
  toggleVideoPictureInPicture,
} from "./src/player/playerPictureInPicture";

describe("isPictureInPictureSupported", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("is false when the document API is missing", () => {
    vi.stubGlobal("document", {});
    expect(isPictureInPictureSupported()).toBe(false);
  });

  it("is false when picture-in-picture is disabled on the document", () => {
    vi.stubGlobal("document", {
      pictureInPictureEnabled: false,
    });
    expect(isPictureInPictureSupported()).toBe(false);
  });
});

describe("toggleVideoPictureInPicture", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("requests picture-in-picture on the video element", async () => {
    const requestPictureInPicture = vi.fn().mockResolvedValue(undefined);
    const video = {
      requestPictureInPicture,
    } as unknown as HTMLVideoElement;
    vi.stubGlobal("document", {
      pictureInPictureElement: null,
      exitPictureInPicture: vi.fn(),
    });

    await toggleVideoPictureInPicture(video);

    expect(requestPictureInPicture).toHaveBeenCalledTimes(1);
  });
});
