import type { ReactNode } from "react";

import type { AnkiLaunchStatus, DueSummary } from "../api/types";
import settingsOutline from "../theme/icons/setting-outline.png";

export type AppRoute = "library" | "roadmap" | "report" | "review" | "settings";

export type AppShellProps = {
  route: AppRoute;
  screenTitle: string;
  onNavigate: (route: AppRoute) => void;
  due: DueSummary;
  anki: AnkiLaunchStatus;
  onStartReview: () => void;
  studyBusy?: boolean;
  children: ReactNode;
};

export function AppShell({
  route,
  screenTitle,
  onNavigate,
  due,
  onStartReview,
  studyBusy = false,
  children,
}: AppShellProps) {
  const studyDisabled = studyBusy;

  return (
    <div className="app-shell">
      <header className="top-bar">
        <div className="top-bar__brand">
          <span className="top-bar__wordmark">Lessonbinder</span>
          <h1 className="top-bar__title">{screenTitle}</h1>
        </div>
        <div className="top-bar__end">
          <div className="due-counts" aria-label="Lessonbinder due counts">
            <span className="due-count">
              <span className="due-count__value">{due.new}</span>
              <span className="due-count__label">New</span>
            </span>
            <span className="due-count">
              <span className="due-count__value">{due.learning}</span>
              <span className="due-count__label">Learning</span>
            </span>
            <span className="due-count">
              <span className="due-count__value">{due.review}</span>
              <span className="due-count__label">Review</span>
            </span>
          </div>
          <div className="study-action">
            <button
              type="button"
              className="study-button"
              disabled={studyDisabled}
              onClick={() => onStartReview()}
            >
              Study
            </button>
          </div>
        </div>
      </header>
      <div className="app-body">
        <nav className="side-nav" aria-label="Main">
          <button
            type="button"
            className={
              route === "library" ? "nav-item nav-item--active" : "nav-item"
            }
            aria-current={route === "library" ? "page" : undefined}
            onClick={() => onNavigate("library")}
          >
            Library
          </button>
          <button
            type="button"
            className={
              route === "roadmap" ? "nav-item nav-item--active" : "nav-item"
            }
            aria-current={route === "roadmap" ? "page" : undefined}
            onClick={() => onNavigate("roadmap")}
          >
            Roadmap
          </button>
          <button
            type="button"
            className={
              route === "report" ? "nav-item nav-item--active" : "nav-item"
            }
            aria-current={route === "report" ? "page" : undefined}
            onClick={() => onNavigate("report")}
          >
            Report
          </button>
          <button
            type="button"
            className={`nav-item theme-icon-label${route === "settings" ? " nav-item--active" : ""}`}
            aria-current={route === "settings" ? "page" : undefined}
            onClick={() => onNavigate("settings")}
          >
            <img
              src={settingsOutline}
              alt=""
              aria-hidden
              className="theme-icon-label__icon"
            />
            Settings
          </button>
        </nav>
        <main className="main-pane">{children}</main>
      </div>
    </div>
  );
}
