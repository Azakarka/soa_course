INSERT INTO likes (post_id, user_id)
VALUES ($1, $2)
RETURNING $table_fields;
