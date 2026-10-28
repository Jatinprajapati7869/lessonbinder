export type VaultUnavailableScreenProps = {
  path: string;
  error: string | null;
  onRetry: () => void;
  busy?: boolean;
};

export function VaultUnavailableScreen({
  path,
  error,
  onRetry,
  busy = false,
}: VaultUnavailableScreenProps) {
  const message =
    error ??
    "Could not open the vault database. Check permissions and disk space.";

  return (
    <div className="vault-unavailable">
      <h1 className="screen-title">AuraVault</h1>
      <p className="library-error" role="alert">
        {message}
      </p>
      <p className="vault-unavailable__path">{path}</p>
      <button
        type="button"
        className="library-button-primary"
        disabled={busy}
        onClick={onRetry}
      >
        Retry
      </button>
    </div>
  );
}
