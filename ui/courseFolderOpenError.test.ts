import { describe, expect, it } from "vitest";

import { invokeErrorMessage } from "./src/api/invokeError";

export const OPENER_OPEN_PATH_FAILED_MESSAGE =
  "Could not open that folder in your file manager.";

describe("course folder open errors", () => {
  it("maps opener_open_path_failed to one sentence without stack text", () => {
    const message = invokeErrorMessage(
      new Error(
        JSON.stringify({
          code: "opener_open_path_failed",
          message: OPENER_OPEN_PATH_FAILED_MESSAGE,
        })
      ),
      "Could not open course folder."
    );
    expect(message).toBe(OPENER_OPEN_PATH_FAILED_MESSAGE);
    expect(message).not.toMatch(/\n/);
    expect(message).not.toMatch(/stack/i);
  });
});
