CREATE TABLE if not exists likes  (
	id				BIGSERIAL PRIMARY KEY,
	post_id			UUID,
	user_id			UUID,
	created_at 		TIMESTAMPTZ DEFAULT NOW()
);
