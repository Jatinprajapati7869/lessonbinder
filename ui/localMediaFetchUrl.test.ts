import { describe, expect, it } from "vitest";

import { isAllowedLocalMediaFetchUrl } from "./src/media/localMediaFetchUrl";

describe("isAllowedLocalMediaFetchUrl", () => {
  it("allows asset and localhost media URLs only", () => {
    expect(isAllowedLocalMediaFetchUrl("asset://chapter.md")).toBe(true);
    expect(isAllowedLocalMediaFetchUrl("http://localhost:5173/x")).toBe(true);
    expect(isAllowedLocalMediaFetchUrl("https://asset.localhost/file")).toBe(
      true
    );
    expect(isAllowedLocalMediaFetchUrl("https://example.com/x")).toBe(false);
  });
});
