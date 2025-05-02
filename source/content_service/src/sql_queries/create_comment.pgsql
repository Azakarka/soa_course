INSERT INTO comments (post_id, user_id, text)
VALUES ($1, $2, $3)
RETURNING $table_fields;
