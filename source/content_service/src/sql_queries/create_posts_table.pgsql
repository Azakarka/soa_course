CREATE TABLE if not exists wall_posts  (
	id				BIGSERIAL PRIMARY KEY,
	post_id			UUID DEFAULT gen_random_uuid(),
	title 			VARCHAR(100),
	description 	TEXT,
	creator_id 		UUID,
	created_at 		TIMESTAMPTZ DEFAULT NOW(),
	updated_at 		TIMESTAMPTZ DEFAULT NOW(),
	is_private 		BOOLEAN DEFAULT TRUE,
	tags			TEXT[]
);
