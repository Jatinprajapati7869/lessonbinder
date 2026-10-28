import type { KeyboardEvent } from "react";

import { handleTableArrowKeys } from "../keyboard/tableListNavigation";

import type { LibrarySearchHit } from "../api/librarySearchTypes";

export const LIBRARY_SEARCH_DEBOUNCE_MS = 300;

export type LibrarySearchResultsProps = {
  hits: LibrarySearchHit[];
  searching: boolean;
  query: string;
  onSelect: (hit: LibrarySearchHit) => void;
};

export function LibrarySearchResults({
  hits,
  searching,
  query,
  onSelect,
}: LibrarySearchResultsProps) {
  const trimmed = query.trim();
  if (trimmed.length === 0) {
    return null;
  }

  if (!searching && hits.length === 0) {
    return <p className="library-search__empty">No matches.</p>;
  }

  if (hits.length === 0) {
    return null;
  }

  function handleResultsKeyDown(event: KeyboardEvent<HTMLUListElement>) {
    handleTableArrowKeys(
      event,
      event.currentTarget,
      "li",
      ".library-search__hit"
    );
  }

  return (
    <ul
      className="library-search__results"
      aria-label="Search results"
      onKeyDown={handleResultsKeyDown}
    >
      {hits.map((hit) => (
        <li key={`${hit.lesson_id}-${hit.match_kind}-${hit.snippet}`}>
          <button
            type="button"
            className="library-search__hit"
            onClick={() => onSelect(hit)}
          >
            <span className="library-search__hit-lesson">
              {hit.lesson_title}
            </span>
            <span className="library-search__hit-course">
              {hit.course_title}
            </span>
            <span className="library-search__hit-snippet">{hit.snippet}</span>
          </button>
        </li>
      ))}
    </ul>
  );
}
