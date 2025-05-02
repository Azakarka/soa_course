INSERT INTO user_profiles (first_name, last_name, email, username, password, phone_number, birth_date)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING $table_fields;
