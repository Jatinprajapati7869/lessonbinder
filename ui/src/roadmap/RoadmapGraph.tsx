import type { DerivedStatus } from "../api/libraryTypes";
import type { RoadmapGraph as RoadmapGraphData } from "../api/roadmapTypes";
import { formatRemainingMs, formatStatus } from "../library/format";

const NODE_WIDTH = 148;
const NODE_HEIGHT = 64;
const H_GAP = 56;
const PADDING = 24;

type NodeLayout = {
  stepId: string;
  x: number;
  y: number;
};

function statusClass(status: DerivedStatus): string {
  switch (status) {
    case "not_started":
      return "roadmap-node__status--not-started";
    case "in_progress":
      return "roadmap-node__status--in-progress";
    case "done":
      return "roadmap-node__status--done";
    case "broken":
      return "roadmap-node__status--broken";
    default:
      return "roadmap-node__status--not-started";
  }
}

function layoutNodes(nodes: RoadmapGraphData["nodes"]): NodeLayout[] {
  return nodes.map((node, index) => ({
    stepId: node.step_id,
    x: PADDING + index * (NODE_WIDTH + H_GAP),
    y: PADDING,
  }));
}

function nodeCenter(layout: NodeLayout): { cx: number; cy: number } {
  return {
    cx: layout.x + NODE_WIDTH / 2,
    cy: layout.y + NODE_HEIGHT / 2,
  };
}

export type RoadmapGraphProps = {
  graph: RoadmapGraphData;
  selectedStepId: string | null;
  onSelectStep: (stepId: string) => void;
};

export function RoadmapGraph({
  graph,
  selectedStepId,
  onSelectStep,
}: RoadmapGraphProps) {
  const layouts = layoutNodes(graph.nodes);
  const layoutById = new Map(layouts.map((l) => [l.stepId, l]));
  const width =
    graph.nodes.length === 0
      ? 320
      : PADDING * 2 +
        graph.nodes.length * NODE_WIDTH +
        (graph.nodes.length - 1) * H_GAP;
  const height = NODE_HEIGHT + PADDING * 2;

  return (
    <svg
      className="roadmap-graph"
      data-testid="roadmap-graph"
      width={width}
      height={height}
      role="group"
      aria-label="Course roadmap graph"
    >
      <defs>
        <marker
          id="roadmap-arrow"
          markerWidth="8"
          markerHeight="8"
          refX="7"
          refY="4"
          orient="auto"
        >
          <path d="M0,0 L8,4 L0,8 Z" className="roadmap-graph__arrowhead" />
        </marker>
      </defs>
      {graph.edges.map((edge) => {
        const from = layoutById.get(edge.from_step_id);
        const to = layoutById.get(edge.to_step_id);
        if (!from || !to) {
          return null;
        }
        const fromCenter = nodeCenter(from);
        const toCenter = nodeCenter(to);
        const x1 = from.x + NODE_WIDTH;
        const y1 = fromCenter.cy;
        const x2 = to.x;
        const y2 = toCenter.cy;
        const isPrerequisite = edge.kind === "prerequisite";
        return (
          <line
            key={`${edge.from_step_id}-${edge.to_step_id}-${edge.kind}`}
            data-testid={`roadmap-edge-${edge.from_step_id}-${edge.to_step_id}`}
            className={
              isPrerequisite
                ? "roadmap-graph__edge roadmap-graph__edge--prerequisite"
                : "roadmap-graph__edge"
            }
            x1={x1}
            y1={y1}
            x2={x2}
            y2={y2}
            markerEnd={isPrerequisite ? undefined : "url(#roadmap-arrow)"}
          />
        );
      })}
      {graph.nodes.map((node) => {
        const layout = layoutById.get(node.step_id);
        if (!layout) {
          return null;
        }
        const selected = selectedStepId === node.step_id;
        const remainingLabel =
          node.remaining_ms === null
            ? "Time unknown"
            : formatRemainingMs(node.remaining_ms);
        return (
          <g
            key={node.step_id}
            data-testid={`roadmap-node-${node.step_id}`}
            className="roadmap-node"
            transform={`translate(${layout.x}, ${layout.y})`}
            role="button"
            tabIndex={0}
            aria-pressed={selected}
            aria-label={`${node.course_title}, ${formatStatus(node.status)}`}
            onClick={() => onSelectStep(node.step_id)}
            onKeyDown={(event) => {
              if (event.key === "Enter" || event.key === " ") {
                event.preventDefault();
                onSelectStep(node.step_id);
              }
            }}
          >
            <rect
              className={
                selected
                  ? "roadmap-node__box roadmap-node__box--selected"
                  : "roadmap-node__box"
              }
              width={NODE_WIDTH}
              height={NODE_HEIGHT}
              rx={4}
            />
            <text className="roadmap-node__title" x={12} y={22}>
              {node.course_title.length > 18
                ? `${node.course_title.slice(0, 16)}…`
                : node.course_title}
            </text>
            <text
              className={`roadmap-node__status ${statusClass(node.status)}`}
              x={12}
              y={40}
            >
              <tspan className="roadmap-node__status-dot" aria-hidden="true">
                ●{" "}
              </tspan>
              {formatStatus(node.status)}
            </text>
            <text className="roadmap-node__remaining" x={12} y={54}>
              {remainingLabel}
            </text>
          </g>
        );
      })}
    </svg>
  );
}
