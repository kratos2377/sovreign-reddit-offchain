use sea_orm::{entity::prelude::*, SelectTwo, SelectTwoMany};
use serde::{Deserialize, Serialize};

use crate::{comments, posts, subreddit};

// Users Entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sov_id: String,
    pub username: String,
        pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_joined_subs::Entity")]
    UserJoinedSubs,
    #[sea_orm(has_many = "super::posts::Entity")]
    Posts,
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
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

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

// Many-to-Many relation between Users and Sub through UserJoinedSubs
impl Related<super::subreddit::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_joined_subs::Relation::Sub.def()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::user_joined_subs::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}


impl Entity {
    /// Find a user with all their joined subs
    pub async fn find_with_subs(db: &DatabaseConnection, sov_id: &str) -> SelectTwoMany<Entity , subreddit::Entity> {
        Self::find_by_id(sov_id)
            .find_with_related(subreddit::Entity)
    }

    /// Find a user with all their posts
    pub async fn find_with_posts(db: &DatabaseConnection, sov_id: &str) -> SelectTwoMany<Entity , posts::Entity> {
        Self::find_by_id(sov_id)
            .find_with_related(posts::Entity)
    }

    /// Find a user with all their comments
    pub async fn find_with_comments(db: &DatabaseConnection, sov_id: &str) -> SelectTwoMany<Entity , comments::Entity> {
        Self::find_by_id(sov_id)
            .find_with_related(comments::Entity)
    }


   
}
