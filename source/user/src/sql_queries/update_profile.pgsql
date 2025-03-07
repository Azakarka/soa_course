UPDATE user_profiles SET
first_name = $2,
last_name = $3,
email = $4,
phone_number = $5,
birth_date = $6
WHERE user_uuid = $1
RETURNING $table_fields;
