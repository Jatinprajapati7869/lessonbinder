import { describe, expect, it } from "vitest";

import {
  invokeErrorMessage,
  parseCommandErrorPayload,
} from "./src/api/invokeError";

describe("invokeError", () => {
  it("parses typed command errors from invoke rejections", () => {
    const payload = parseCommandErrorPayload(
      JSON.stringify({
        code: "backup_io_error",
        message: "io error: disk full",
      })
    );
    expect(payload).toEqual({
      code: "backup_io_error",
      message: "io error: disk full",
    });
  });

  it("shows human message for typed errors", () => {
    const message = invokeErrorMessage(
      new Error(
        JSON.stringify({
          code: "rescan_course_not_found",
          message: "course not found",
        })
      ),
      "fallback"
    );
    expect(message).toBe("course not found");
  });
});
