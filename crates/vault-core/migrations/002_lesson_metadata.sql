-- Lesson metadata from deterministic extractors (never invented summaries).

ALTER TABLE lesson ADD COLUMN duration_ms INTEGER;
ALTER TABLE lesson ADD COLUMN duration_extractor TEXT;
ALTER TABLE lesson ADD COLUMN duration_error TEXT;

ALTER TABLE lesson ADD COLUMN page_count INTEGER;
ALTER TABLE lesson ADD COLUMN page_extractor TEXT;
ALTER TABLE lesson ADD COLUMN page_error TEXT;

ALTER TABLE lesson ADD COLUMN heading TEXT;
ALTER TABLE lesson ADD COLUMN heading_extractor TEXT;
ALTER TABLE lesson ADD COLUMN heading_error TEXT;

ALTER TABLE lesson ADD COLUMN subtitle_cue_count INTEGER;
ALTER TABLE lesson ADD COLUMN subtitle_first_ms INTEGER;
ALTER TABLE lesson ADD COLUMN subtitle_last_ms INTEGER;
ALTER TABLE lesson ADD COLUMN subtitle_extractor TEXT;
ALTER TABLE lesson ADD COLUMN subtitle_error TEXT;
