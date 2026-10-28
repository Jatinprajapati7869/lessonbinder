/** Whether `fetch` may load lesson media converted for this app (never third-party hosts). */
export function isAllowedLocalMediaFetchUrl(url: string): boolean {
  if (url.startsWith("asset://")) {
    return true;
  }
  try {
    const { hostname, protocol } = new URL(url);
    if (protocol !== "http:" && protocol !== "https:") {
      return false;
    }
    return (
      hostname === "localhost" ||
      hostname === "127.0.0.1" ||
      hostname === "asset.localhost"
    );
  } catch {
    return false;
  }
}
