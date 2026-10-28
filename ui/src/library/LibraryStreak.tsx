export type LibraryStreakProps = {
  days: number;
};

export function LibraryStreak({ days }: LibraryStreakProps) {
  return (
    <p className="library-streak" aria-label={`Study streak ${days} days`}>
      {days} days
    </p>
  );
}
