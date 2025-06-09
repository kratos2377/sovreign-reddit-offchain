

pub fn get_sql_statement_for_post_upvote(value: i32 , prev_value: i32) -> &'static str {


 if prev_value == 0 {

    if value == 1 {

             r#"
            UPDATE posts 
            SET 
                upvote = upvote + 1,
                score = (upvote + 1) - downvote,
                  updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#

    } else {
         r#"
            UPDATE posts 
            SET 
                downvote = downvote + 1,
                score = upvote - (downvote + 1),
                updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#
    }

 } else if prev_value == 1 {

    if value == 0 {
         r#"
            UPDATE posts 
            SET 
                upvote = GREATEST(upvote - 1, 0),
                score = GREATEST(upvote - 1, 0) - downvote,
                updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#
    } else {


         r#"
            UPDATE posts 
            SET 
                upvote = GREATEST(upvote - 1, 0),
                downvote = downvote + 1,
                score = GREATEST(upvote - 1, 0) - downvote,
                updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#

    }


 } else {


    if value ==  0 {

               r#"
            UPDATE posts 
            SET 
                downvote = GREATEST(downvote - 1, 0),
                score = upvote - GREATEST(downvote - 1, 0),
                updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#

    } else {
           r#"
            UPDATE posts 
            SET 
                downvote = GREATEST(downvote - 1, 0),
                upvote = upvote + 1,
                score = upvote - GREATEST(downvote - 1, 0),
                updated_at = $2
            WHERE post_sov_id = $1
            RETURNING *
            "#
    }

 }



}