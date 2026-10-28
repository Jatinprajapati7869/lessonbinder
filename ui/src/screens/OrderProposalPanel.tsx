import type { OrderProposal } from "../api/courseTypes";

export type OrderProposalPanelProps = {
  proposal: OrderProposal;
  busy: boolean;
  onAccept: () => void;
  onReject: () => void;
};

export function OrderProposalPanel({
  proposal,
  busy,
  onAccept,
  onReject,
}: OrderProposalPanelProps) {
  return (
    <section className="order-proposal" aria-label="Order proposal">
      <h2 className="order-proposal__title">Proposed lesson order</h2>
      <div className="order-proposal__columns">
        <div>
          <h3 className="order-proposal__heading">Current path order</h3>
          <ol className="order-proposal__list">
            {proposal.current_titles.map((title, index) => (
              <li key={`current-${index}-${title}`}>{title}</li>
            ))}
          </ol>
        </div>
        <div>
          <h3 className="order-proposal__heading">Proposed order</h3>
          <ol className="order-proposal__list">
            {proposal.proposed_titles.map((title, index) => (
              <li key={`proposed-${index}-${title}`}>{title}</li>
            ))}
          </ol>
        </div>
      </div>
      <div className="library-toolbar">
        <button
          type="button"
          className="study-button"
          disabled={busy}
          onClick={onAccept}
        >
          Accept
        </button>
        <button
          type="button"
          className="library-button-secondary"
          disabled={busy}
          onClick={onReject}
        >
          Reject
        </button>
      </div>
    </section>
  );
}
