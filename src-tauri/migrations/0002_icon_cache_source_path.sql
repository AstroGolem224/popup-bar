-- Add source_path for cache invalidation: when source no longer exists, cache entry is ignored.
ALTER TABLE icon_cache ADD COLUMN source_path TEXT NOT NULL DEFAULT '';
