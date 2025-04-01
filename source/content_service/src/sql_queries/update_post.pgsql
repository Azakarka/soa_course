UPDATE wall_posts SET
title = $2,
description = $3,
is_private = $4,
tags = $5
WHERE post_id = $1
RETURNING $table_fields;
