import type { LibraryVisibility } from "../api/libraryTypes";

export type LibraryVisibilityFilterProps = {
  value: LibraryVisibility;
  onChange: (value: LibraryVisibility) => void;
};

export function LibraryVisibilityFilter({
  value,
  onChange,
}: LibraryVisibilityFilterProps) {
  return (
    <div
      className="library-visibility-filter"
      role="group"
      aria-label="Course archive filter"
    >
      <label className="library-visibility-filter__option">
        <input
          type="radio"
          name="library-visibility"
          value="active"
          checked={value === "active"}
          onChange={() => onChange("active")}
        />
        Active
      </label>
      <label className="library-visibility-filter__option">
        <input
          type="radio"
          name="library-visibility"
          value="archived"
          checked={value === "archived"}
          onChange={() => onChange("archived")}
        />
        Archived
      </label>
    </div>
  );
}
