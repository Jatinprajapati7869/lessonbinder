export type CommandErrorPayload = {
  code: string;
  message: string;
};

export function parseCommandErrorPayload(
  text: string
): CommandErrorPayload | null {
  try {
    const parsed = JSON.parse(text) as Partial<CommandErrorPayload>;
    if (typeof parsed.code === "string" && typeof parsed.message === "string") {
      return { code: parsed.code, message: parsed.message };
    }
  } catch {
    /* not JSON */
  }
  return null;
}

export function invokeErrorMessage(raw: unknown, fallback: string): string {
  if (raw instanceof Error) {
    const parsed = parseCommandErrorPayload(raw.message);
    return parsed?.message ?? raw.message;
  }
  return fallback;
}
