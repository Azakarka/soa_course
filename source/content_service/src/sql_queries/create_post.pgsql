INSERT INTO wall_posts (title, description, creator_id, is_private, tags)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields;
