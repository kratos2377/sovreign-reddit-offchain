use uuid::Uuid;

    
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{posts, subreddit, users};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_liked_posts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_sov_id: String,
    pub post_sov_id: String,
    pub value: i32,
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
        belongs_to = "posts::Entity",
        from = "Column::PostSovId",
        to = "posts::Column::PostSovId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post,
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


impl Entity {
        pub fn find_by_post_and_user_sov_id(post_sov_id: &str , user_sov_id: &str) -> Select<Entity> {
        Self::find().filter(Column::PostSovId.eq(post_sov_id))
            .filter(Column::UserSovId.eq(user_sov_id))
    }
}