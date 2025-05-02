CREATE TABLE if not exists comments  (
	id				BIGSERIAL PRIMARY KEY,
	comment_id 		UUID DEFAULT gen_random_uuid(),
	post_id			UUID,
	user_id			UUID,
	text 			TEXT,
	created_at 		TIMESTAMPTZ DEFAULT NOW()
);
