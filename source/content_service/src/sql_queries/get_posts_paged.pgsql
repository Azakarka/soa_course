SELECT $table_fields
FROM wall_posts
WHERE creator_id=$1
AND (is_private=false OR creator_id=$4)
ORDER BY updated_at DESC
LIMIT $2 OFFSET $3
