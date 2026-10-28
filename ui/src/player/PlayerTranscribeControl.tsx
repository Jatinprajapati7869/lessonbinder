export type TranscribeJobState = "idle" | "running" | "failed" | "done";

export type PlayerTranscribeControlProps = {
  modelPath: string;
  jobState: TranscribeJobState;
  errorMessage: string | null;
  onTranscribe: () => void;
};

export const TRANSCRIPTION_MODEL_PATH_REQUIRED =
  "A local transcription model path is required. Add it in Settings.";

export function PlayerTranscribeControl({
  modelPath,
  jobState,
  errorMessage,
  onTranscribe,
}: PlayerTranscribeControlProps) {
  const pathMissing = modelPath.trim() === "";

  return (
    <div className="player-transcribe" data-testid="player-transcribe">
      <button
        type="button"
        className="library-button-secondary"
        disabled={jobState === "running"}
        onClick={onTranscribe}
      >
        Transcribe
      </button>
      {pathMissing ? (
        <p className="player-transcribe__hint" role="status">
          {TRANSCRIPTION_MODEL_PATH_REQUIRED}
        </p>
      ) : null}
      {jobState === "running" ? (
        <p className="player-transcribe__status" role="status">
          Transcription running.
        </p>
      ) : null}
      {jobState === "done" ? (
        <p className="player-transcribe__status" role="status">
          Transcription done.
        </p>
      ) : null}
      {jobState === "failed" && errorMessage ? (
        <p className="player-transcribe__error" role="alert">
          {errorMessage}
        </p>
      ) : null}
    </div>
  );
}
