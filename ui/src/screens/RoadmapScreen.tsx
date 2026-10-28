import { useCallback, useEffect, useState } from "react";

import type { CourseRow } from "../api/libraryTypes";
import type {
  RoadmapDetail,
  RoadmapListEntry,
  RoadmapStep,
} from "../api/roadmapTypes";
import { useLibraryCommands } from "../library/useLibraryCommands";
import { formatRemainingMs, formatStatus } from "../library/format";
import { RoadmapGraph } from "../roadmap/RoadmapGraph";
import { RoadmapPrerequisitePanel } from "../roadmap/RoadmapPrerequisitePanel";
import { useRoadmapCommands } from "../roadmap/useRoadmapCommands";

const DEFAULT_ROADMAP_TITLE = "Study path";

export function RoadmapScreen() {
  const roadmapCommands = useRoadmapCommands();
  const libraryCommands = useLibraryCommands();
  const [roadmaps, setRoadmaps] = useState<RoadmapListEntry[]>([]);
  const [activeRoadmapId, setActiveRoadmapId] = useState<string | null>(null);
  const [detail, setDetail] = useState<RoadmapDetail | null>(null);
  const [courses, setCourses] = useState<CourseRow[]>([]);
  const [selectedStepId, setSelectedStepId] = useState<string | null>(null);
  const [pickCourseId, setPickCourseId] = useState("");
  const [noteDraft, setNoteDraft] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [prereqFromStepId, setPrereqFromStepId] = useState("");
  const [prereqToStepId, setPrereqToStepId] = useState("");
  const [relinkCourseId, setRelinkCourseId] = useState("");

  const refreshList = useCallback(async () => {
    return roadmapCommands.listRoadmaps();
  }, [roadmapCommands]);

  const refreshDetail = useCallback(async () => {
    if (!activeRoadmapId) {
      setDetail(null);
      return;
    }
    setError(null);
    try {
      const next = await roadmapCommands.getRoadmapDetail(activeRoadmapId);
      setDetail(next);
      if (
        selectedStepId &&
        !next.steps.some((s) => s.step_id === selectedStepId)
      ) {
        setSelectedStepId(null);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not load roadmap.");
    }
  }, [roadmapCommands, activeRoadmapId, selectedStepId]);

  useEffect(() => {
    void refreshList()
      .then((entries) => {
        setRoadmaps(entries);
        if (entries.length === 0) {
          setActiveRoadmapId(null);
          setDetail(null);
          return;
        }
        setActiveRoadmapId((current) => {
          if (current && entries.some((e) => e.roadmap_id === current)) {
            return current;
          }
          return entries[0].roadmap_id;
        });
      })
      .catch(() => {
        setError("Could not load roadmaps.");
      });
  }, [refreshList]);

  useEffect(() => {
    void refreshDetail();
  }, [refreshDetail]);

  useEffect(() => {
    libraryCommands
      .listCourses("active")
      .then(setCourses)
      .catch(() => setCourses([]));
  }, [libraryCommands]);

  const selectedStep: RoadmapStep | null =
    detail?.steps.find((s) => s.step_id === selectedStepId) ?? null;

  useEffect(() => {
    setNoteDraft(selectedStep?.note ?? "");
    setRelinkCourseId("");
  }, [selectedStep?.step_id, selectedStep?.note]);

  async function handleCreateRoadmap() {
    setBusy(true);
    setError(null);
    try {
      const id = await roadmapCommands.createRoadmap(DEFAULT_ROADMAP_TITLE);
      const entries = await refreshList();
      setRoadmaps(entries);
      setActiveRoadmapId(id);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not create roadmap.");
    } finally {
      setBusy(false);
    }
  }

  async function handleAddCourse() {
    if (!activeRoadmapId || !pickCourseId) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await roadmapCommands.addRoadmapStep(activeRoadmapId, pickCourseId);
      setPickCourseId("");
      await refreshDetail();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not add course.");
    } finally {
      setBusy(false);
    }
  }

  async function moveStep(stepId: string, direction: "up" | "down") {
    if (!detail || !activeRoadmapId) {
      return;
    }
    const ids = detail.steps.map((s) => s.step_id);
    const index = ids.indexOf(stepId);
    if (index < 0) {
      return;
    }
    const target = direction === "up" ? index - 1 : index + 1;
    if (target < 0 || target >= ids.length) {
      return;
    }
    const next = [...ids];
    const [removed] = next.splice(index, 1);
    next.splice(target, 0, removed);
    setBusy(true);
    try {
      await roadmapCommands.reorderRoadmapSteps(activeRoadmapId, next);
      await refreshDetail();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not reorder steps.");
    } finally {
      setBusy(false);
    }
  }

  async function handleAddPrerequisite() {
    if (
      !prereqFromStepId ||
      !prereqToStepId ||
      prereqFromStepId === prereqToStepId
    ) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await roadmapCommands.addPrerequisiteEdge(
        prereqFromStepId,
        prereqToStepId
      );
      setPrereqFromStepId("");
      setPrereqToStepId("");
      await refreshDetail();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not add prerequisite.");
    } finally {
      setBusy(false);
    }
  }

  async function handleRemovePrerequisite(
    fromStepId: string,
    toStepId: string
  ) {
    setBusy(true);
    setError(null);
    try {
      await roadmapCommands.removePrerequisiteEdge(fromStepId, toStepId);
      await refreshDetail();
    } catch (e) {
      setError(
        e instanceof Error ? e.message : "Could not remove prerequisite."
      );
    } finally {
      setBusy(false);
    }
  }

  async function handleRelinkStep() {
    if (!selectedStep || !relinkCourseId) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await roadmapCommands.relinkRoadmapStep(
        selectedStep.step_id,
        relinkCourseId
      );
      setRelinkCourseId("");
      await refreshDetail();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not relink step.");
    } finally {
      setBusy(false);
    }
  }

  async function handleExportSvg() {
    if (!detail) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await roadmapCommands.exportRoadmapGraphSvg(detail.graph, detail.title);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not export SVG.");
    } finally {
      setBusy(false);
    }
  }

  async function saveNote() {
    if (!selectedStep) {
      return;
    }
    const normalized = noteDraft.trim() === "" ? null : noteDraft.trim();
    if (normalized === (selectedStep.note ?? null)) {
      return;
    }
    setBusy(true);
    try {
      await roadmapCommands.updateRoadmapStepNote(
        selectedStep.step_id,
        normalized
      );
      await refreshDetail();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not save note.");
    } finally {
      setBusy(false);
    }
  }

  if (roadmaps.length === 0) {
    return (
      <div className="roadmap-screen">
        <p className="library-empty__hint">No roadmap yet.</p>
        <p className="library-empty__hint">
          Connect courses in the order you want to study them.
        </p>
        {error ? (
          <p className="library-error" role="alert">
            {error}
          </p>
        ) : null}
        <button
          type="button"
          className="library-button-primary"
          disabled={busy}
          onClick={() => void handleCreateRoadmap()}
        >
          Create roadmap
        </button>
      </div>
    );
  }

  const emptySteps = detail !== null && detail.steps.length === 0;
  const prerequisiteEdges =
    detail?.graph.edges.filter((edge) => edge.kind === "prerequisite") ?? [];

  return (
    <div className="roadmap-screen">
      <header className="roadmap-header">
        <h1 className="screen-title">{detail?.title ?? "Roadmap"}</h1>
        {detail ? (
          <div className="roadmap-header__meta">
            <span className="library-status">
              {formatStatus(detail.status)}
            </span>
            <span className="roadmap-header__remaining">
              {detail.unknown_duration_count > 0
                ? "Remaining time includes unknown durations"
                : `${formatRemainingMs(detail.remaining_ms)} left`}
            </span>
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy || detail.steps.length === 0}
              onClick={() => void handleExportSvg()}
            >
              Export SVG
            </button>
          </div>
        ) : null}
      </header>

      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}

      {emptySteps ? (
        <div className="roadmap-empty-steps">
          <p className="library-empty__hint">
            Add a course from your library as the first step.
          </p>
          <div className="roadmap-add-course">
            <label
              className="roadmap-add-course__label"
              htmlFor="roadmap-course-pick"
            >
              Course
            </label>
            <select
              id="roadmap-course-pick"
              className="roadmap-add-course__select"
              value={pickCourseId}
              onChange={(e) => setPickCourseId(e.target.value)}
              disabled={busy || courses.length === 0}
            >
              <option value="">Select a course</option>
              {courses.map((c) => (
                <option key={c.course_id} value={c.course_id}>
                  {c.title}
                </option>
              ))}
            </select>
            <button
              type="button"
              className="library-button-primary"
              disabled={busy || !pickCourseId}
              onClick={() => void handleAddCourse()}
            >
              Add course
            </button>
          </div>
        </div>
      ) : detail ? (
        <div className="roadmap-layout">
          <div className="roadmap-canvas">
            <RoadmapGraph
              graph={detail.graph}
              selectedStepId={selectedStepId}
              onSelectStep={setSelectedStepId}
            />
          </div>
          <aside className="roadmap-panel" aria-label="Step details">
            <div className="roadmap-panel__actions">
              <div className="roadmap-add-course roadmap-add-course--inline">
                <label
                  className="roadmap-add-course__label"
                  htmlFor="roadmap-course-add"
                >
                  Add course
                </label>
                <select
                  id="roadmap-course-add"
                  className="roadmap-add-course__select"
                  value={pickCourseId}
                  onChange={(e) => setPickCourseId(e.target.value)}
                  disabled={busy}
                >
                  <option value="">Select a course</option>
                  {courses.map((c) => (
                    <option key={c.course_id} value={c.course_id}>
                      {c.title}
                    </option>
                  ))}
                </select>
                <button
                  type="button"
                  className="library-button-secondary"
                  disabled={busy || !pickCourseId}
                  onClick={() => void handleAddCourse()}
                >
                  Add
                </button>
              </div>
            </div>
            {selectedStep ? (
              <>
                <h2 className="roadmap-panel__title">
                  {selectedStep.course_title}
                </h2>
                <p className="roadmap-panel__status">
                  {formatStatus(selectedStep.status)}
                </p>
                {selectedStep.status === "broken" ? (
                  <div className="roadmap-relink">
                    <p className="library-empty__hint">
                      This step lost its course. Relink it to a course in your
                      library.
                    </p>
                    <label
                      className="roadmap-add-course__label"
                      htmlFor="roadmap-relink-course"
                    >
                      Course
                    </label>
                    <select
                      id="roadmap-relink-course"
                      className="roadmap-add-course__select"
                      value={relinkCourseId}
                      onChange={(e) => setRelinkCourseId(e.target.value)}
                      disabled={busy}
                    >
                      <option value="">Select a course</option>
                      {courses.map((c) => (
                        <option key={c.course_id} value={c.course_id}>
                          {c.title}
                        </option>
                      ))}
                    </select>
                    <button
                      type="button"
                      className="library-button-primary"
                      disabled={busy || !relinkCourseId}
                      onClick={() => void handleRelinkStep()}
                    >
                      Relink
                    </button>
                  </div>
                ) : null}
                <div className="roadmap-panel__reorder">
                  <button
                    type="button"
                    className="library-button-secondary"
                    disabled={busy}
                    onClick={() => void moveStep(selectedStep.step_id, "up")}
                  >
                    Move up
                  </button>
                  <button
                    type="button"
                    className="library-button-secondary"
                    disabled={busy}
                    onClick={() => void moveStep(selectedStep.step_id, "down")}
                  >
                    Move down
                  </button>
                </div>
                <label
                  className="roadmap-panel__note-label"
                  htmlFor="roadmap-step-note"
                >
                  Note
                </label>
                <textarea
                  id="roadmap-step-note"
                  className="roadmap-panel__note"
                  rows={5}
                  value={noteDraft}
                  onChange={(e) => setNoteDraft(e.target.value)}
                  onBlur={() => void saveNote()}
                  disabled={busy}
                />
              </>
            ) : (
              <p className="library-empty__hint">
                Select a step to edit its note.
              </p>
            )}
            <RoadmapPrerequisitePanel
              steps={detail.steps}
              prerequisiteEdges={prerequisiteEdges}
              prereqFromStepId={prereqFromStepId}
              prereqToStepId={prereqToStepId}
              busy={busy}
              onPrereqFromChange={setPrereqFromStepId}
              onPrereqToChange={setPrereqToStepId}
              onAddPrerequisite={() => void handleAddPrerequisite()}
              onRemovePrerequisite={(from, to) =>
                void handleRemovePrerequisite(from, to)
              }
            />
            <ol className="roadmap-step-list">
              {detail.steps.map((step, index) => (
                <li key={step.step_id}>
                  <button
                    type="button"
                    className={
                      step.step_id === selectedStepId
                        ? "roadmap-step-list__item roadmap-step-list__item--active"
                        : "roadmap-step-list__item"
                    }
                    onClick={() => setSelectedStepId(step.step_id)}
                  >
                    <span className="roadmap-step-list__index">
                      {index + 1}
                    </span>
                    <span>{step.course_title}</span>
                    <span className="roadmap-step-list__status">
                      {formatStatus(step.status)}
                    </span>
                  </button>
                </li>
              ))}
            </ol>
          </aside>
        </div>
      ) : null}
    </div>
  );
}
