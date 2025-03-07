CREATE TABLE if not exists user_profiles  (
	id				BIGSERIAL PRIMARY KEY,
	user_uuid 		UUID DEFAULT gen_random_uuid(),
	first_name  	VARCHAR(200),
	last_name   	VARCHAR(200),
	email       	VARCHAR(50) NOT NULL,
	username    	VARCHAR(50) UNIQUE NOT NULL,
	password		VARCHAR(64) NOT NULL,
	phone_number	VARCHAR(15),
	birth_date		TIMESTAMPTZ,
	created_at 		TIMESTAMPTZ DEFAULT NOW(),
	updated_at 		TIMESTAMPTZ DEFAULT NOW()
);
