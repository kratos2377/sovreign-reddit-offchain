
    
use sea_orm::entity::prelude::*;
use sea_orm::SelectTwoMany;
use serde::{Deserialize, Serialize};

use crate::{posts, subreddit};
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "subreddit")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub sub_sov_id: String,
        pub subname: String,
        pub sub_description: String,
        pub mods: Vec<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::user_joined_subs::Entity")]
        UserJoinedSubs,
        #[sea_orm(has_many = "super::posts::Entity")]
        Posts,
    }

    impl Related<super::user_joined_subs::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::UserJoinedSubs.def()
        }
    }

    impl Related<super::posts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Posts.def()
        }
    }

    // Many-to-Many relation between Sub and Users through UserJoinedSubs
    impl Related<super::users::Entity> for Entity {
        fn to() -> RelationDef {
            super::user_joined_subs::Relation::Users.def()
        }
        
        fn via() -> Option<RelationDef> {
            Some(super::user_joined_subs::Relation::Sub.def().rev())
        }
    }

    impl ActiveModelBehavior for ActiveModel {}


    impl Entity {
    /// Find a subreddit with all its posts
    pub async fn find_with_posts(db: &DatabaseConnection, sub_sov_id: &str) -> SelectTwoMany<Entity , posts::Entity> {
        Self::find_by_id(sub_sov_id)
            .find_with_related(posts::Entity)
    }

    // Find a subreddit with all its members
    // pub async fn find_with_members(db: &DatabaseConnection, sub_sov_id: &str) -> Result<Option<(subreddit::Model, Vec<Model>)>, DbErr> {
    //     Self::find_by_id(sub_sov_id)
    //         .find_with_related(Entity)
    //         .one(db)
    //         .await
   // }
}