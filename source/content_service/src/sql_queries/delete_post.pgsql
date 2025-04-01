DELETE FROM wall_posts WHERE post_id = $1 RETURNING $table_fields;
