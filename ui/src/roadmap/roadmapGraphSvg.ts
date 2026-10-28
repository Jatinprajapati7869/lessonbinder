import type { RoadmapGraph } from "../api/roadmapTypes";
import { formatStatus } from "../library/format";

const NODE_WIDTH = 148;
const NODE_HEIGHT = 64;
const H_GAP = 56;
const PADDING = 24;

const EXPORT_FONT = "IBM Plex Sans, system-ui, sans-serif";
const EXPORT_BG_ELEVATED = "#ffffff";
const EXPORT_BORDER = "#e5e4e2";
const EXPORT_TEXT_PRIMARY = "#1c1917";
const EXPORT_TEXT_SECONDARY = "#57534e";
const EXPORT_TEXT_MUTED = "#78716c";

function escapeXml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

function nodePosition(index: number): { x: number; y: number } {
  return {
    x: PADDING + index * (NODE_WIDTH + H_GAP),
    y: PADDING,
  };
}

export function buildRoadmapGraphSvg(graph: RoadmapGraph): string {
  const layoutById = new Map(
    graph.nodes.map((node, index) => [node.step_id, nodePosition(index)])
  );
  const width =
    graph.nodes.length === 0
      ? 320
      : PADDING * 2 +
        graph.nodes.length * NODE_WIDTH +
        (graph.nodes.length - 1) * H_GAP;
  const height = NODE_HEIGHT + PADDING * 2;

  const edgeLines = graph.edges
    .map((edge) => {
      const from = layoutById.get(edge.from_step_id);
      const to = layoutById.get(edge.to_step_id);
      if (!from || !to) {
        return "";
      }
      const x1 = from.x + NODE_WIDTH;
      const y1 = from.y + NODE_HEIGHT / 2;
      const x2 = to.x;
      const y2 = to.y + NODE_HEIGHT / 2;
      const isPrerequisite = edge.kind === "prerequisite";
      const stroke = isPrerequisite ? EXPORT_TEXT_MUTED : EXPORT_TEXT_SECONDARY;
      const dash = isPrerequisite ? ' stroke-dasharray="4 3"' : "";
      const marker = isPrerequisite ? "" : ' marker-end="url(#order-arrow)"';
      return `<line x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" stroke="${stroke}" stroke-width="1.5" fill="none"${dash}${marker} />`;
    })
    .join("\n  ");

  const nodeGroups = graph.nodes
    .map((node) => {
      const layout = layoutById.get(node.step_id);
      if (!layout) {
        return "";
      }
      const title = escapeXml(node.course_title);
      const status = escapeXml(formatStatus(node.status));
      return `<g transform="translate(${layout.x}, ${layout.y})">
    <rect width="${NODE_WIDTH}" height="${NODE_HEIGHT}" rx="4" fill="${EXPORT_BG_ELEVATED}" stroke="${EXPORT_BORDER}" />
    <text x="12" y="22" font-family="${EXPORT_FONT}" font-size="12" font-weight="600" fill="${EXPORT_TEXT_PRIMARY}">${title}</text>
    <text x="12" y="40" font-family="${EXPORT_FONT}" font-size="11" fill="${EXPORT_TEXT_SECONDARY}">${status}</text>
  </g>`;
    })
    .join("\n  ");

  return `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" role="img" aria-label="Roadmap graph">
  <defs>
    <marker id="order-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
      <path d="M0,0 L8,4 L0,8 Z" fill="${EXPORT_TEXT_MUTED}" />
    </marker>
  </defs>
  ${edgeLines}
  ${nodeGroups}
</svg>`;
}
