import type { RoadmapGraphEdge, RoadmapStep } from "../api/roadmapTypes";

export type RoadmapPrerequisitePanelProps = {
  steps: RoadmapStep[];
  prerequisiteEdges: RoadmapGraphEdge[];
  prereqFromStepId: string;
  prereqToStepId: string;
  busy: boolean;
  onPrereqFromChange: (stepId: string) => void;
  onPrereqToChange: (stepId: string) => void;
  onAddPrerequisite: () => void;
  onRemovePrerequisite: (fromStepId: string, toStepId: string) => void;
};

function stepLabel(steps: RoadmapStep[], stepId: string): string {
  return steps.find((s) => s.step_id === stepId)?.course_title ?? stepId;
}

export function RoadmapPrerequisitePanel({
  steps,
  prerequisiteEdges,
  prereqFromStepId,
  prereqToStepId,
  busy,
  onPrereqFromChange,
  onPrereqToChange,
  onAddPrerequisite,
  onRemovePrerequisite,
}: RoadmapPrerequisitePanelProps) {
  if (steps.length < 2) {
    return null;
  }

  return (
    <section className="roadmap-prereq" aria-label="Prerequisites">
      <h3 className="roadmap-prereq__title">Prerequisites</h3>
      <p className="roadmap-prereq__hint">
        The first step must be finished before the second. Study order arrows
        are separate and are not added as prerequisites.
      </p>
      <div className="roadmap-prereq__add">
        <label className="roadmap-prereq__label" htmlFor="roadmap-prereq-from">
          Finish first
        </label>
        <select
          id="roadmap-prereq-from"
          className="roadmap-add-course__select"
          value={prereqFromStepId}
          onChange={(e) => onPrereqFromChange(e.target.value)}
          disabled={busy}
        >
          <option value="">Select step</option>
          {steps.map((step) => (
            <option key={step.step_id} value={step.step_id}>
              {step.course_title}
            </option>
          ))}
        </select>
        <label className="roadmap-prereq__label" htmlFor="roadmap-prereq-to">
          Before starting
        </label>
        <select
          id="roadmap-prereq-to"
          className="roadmap-add-course__select"
          value={prereqToStepId}
          onChange={(e) => onPrereqToChange(e.target.value)}
          disabled={busy}
        >
          <option value="">Select step</option>
          {steps.map((step) => (
            <option key={step.step_id} value={step.step_id}>
              {step.course_title}
            </option>
          ))}
        </select>
        <button
          type="button"
          className="library-button-secondary"
          disabled={busy || !prereqFromStepId || !prereqToStepId}
          onClick={onAddPrerequisite}
        >
          Add prerequisite
        </button>
      </div>
      {prerequisiteEdges.length === 0 ? (
        <p className="library-empty__hint">No prerequisite edges yet.</p>
      ) : (
        <ul className="roadmap-prereq__list">
          {prerequisiteEdges.map((edge) => (
            <li key={`${edge.from_step_id}-${edge.to_step_id}`}>
              <span>
                {stepLabel(steps, edge.from_step_id)} →{" "}
                {stepLabel(steps, edge.to_step_id)}
              </span>
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() =>
                  onRemovePrerequisite(edge.from_step_id, edge.to_step_id)
                }
              >
                Remove
              </button>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
