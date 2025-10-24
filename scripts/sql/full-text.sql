ALTER TABLE topics 
	ADD COLUMN search_vector TSVECTOR GENERATED ALWAYS AS (
		SETWEIGHT(TO_TSVECTOR('chinese', title), 'A') || ' ' || TO_TSVECTOR('chinese', md)
	) STORED;

CREATE INDEX idx_topics_search_vector ON topics USING GIN (search_vector);