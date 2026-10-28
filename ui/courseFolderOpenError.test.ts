import { describe, expect, it } from "vitest";

import { invokeErrorMessage } from "./src/api/invokeError";

describe("course folder open errors", () => {
  it("maps opener_open_path_failed to one sentence without stack text", () => {
    const expected = "Could not open that folder in your file manager.";
    const message = invokeErrorMessage(
      new Error(
        JSON.stringify({
          code: "opener_open_path_failed",
          message: expected,
        })
      ),
      "Could not open course folder."
    );
    expect(message).toBe(expected);
    expect(message).not.toMatch(/\n/);
    expect(message).not.toMatch(/stack/i);
  });
});
