 use sea_orm::{entity::prelude::*, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{comments, user_liked_posts, subreddit, users};
 
 
 #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub post_sov_id: String,
        pub title: String,
        pub content: String,
        pub sub_sov_id: String,
        pub flair: String,
        pub user_sov_id: String,
        pub upvote: i32,
        pub downvote: i32,
        pub score: i32,
            pub created_at: DateTime,
    pub updated_at: DateTime,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "users::Entity",
            from = "Column::UserSovId",
            to = "users::Column::SovId",
            on_update = "Cascade",
            on_delete = "Cascade"
        )]
        Users,
        #[sea_orm(
            belongs_to = "subreddit::Entity",
            from = "Column::SubSovId",
            to = "subreddit::Column::SubSovId",
            on_update = "Cascade",
            on_delete = "Cascade"
        )]
        Sub,
        #[sea_orm(has_many = "comments::Entity")]
        Comments,

        #[sea_orm(has_many = "user_liked_posts::Entity")]
        UserLikedPosts,
    }

    // impl Related<users::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::Users.def()
    //     }
    // }


        impl Related<users::Entity> for Entity {
                fn to() -> RelationDef {
                    user_liked_posts::Relation::Users.def()
                }
                
                fn via() -> Option<RelationDef> {
                    Some(user_liked_posts::Relation::Post.def().rev())
                }
    }

    impl Related<subreddit::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Sub.def()
        }
    }

    impl Related<comments::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Comments.def()
        }
    }

        impl Related<user_liked_posts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::UserLikedPosts.def()
        }
    }





    impl ActiveModelBehavior for ActiveModel {}


    
impl Entity {
 

    /// Find a post with its author and subreddit
    pub async fn find_with_author_and_sub( post_sov_id: &str) -> Select<Entity> {
        Self::find_by_id(post_sov_id)
    }


    pub async fn find_by_user_id(user_id: &str) -> Select<Entity> {
        Self::find().filter(Column::UserSovId.eq(user_id))
    }

     pub async fn find_by_sub_id(sub_id: &str) -> Select<Entity> {
        Self::find().filter(Column::SubSovId.eq(sub_id)).order_by_asc(Column::CreatedAt)
    }
}