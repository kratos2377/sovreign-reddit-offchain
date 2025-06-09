 use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{posts, users}; 
 
 #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "comments")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub user_sov_id: String,
        pub post_sov_id: String,
        pub content: String,
        pub upvote: i32,
        pub downvote: i32,
        pub score: i32,
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
            belongs_to = "posts::Entity",
            from = "Column::PostSovId",
            to = "posts::Column::PostSovId",
            on_update = "Cascade",
            on_delete = "Cascade"
        )]
        Posts,
    }

    impl Related<users::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Users.def()
        }
    }

    impl Related<posts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Posts.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}