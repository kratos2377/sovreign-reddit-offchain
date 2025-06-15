use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{subreddit, users};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sub_mods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sub_sov_id: String,
    pub user_sov_id: String,
    pub subname: String,
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
    Subreddit,
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<subreddit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subreddit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


impl Entity {
    pub fn find_by_user_id(user_sov_id: &str) -> Select<Entity> {
        Self::find()
            .filter(Column::UserSovId.eq(user_sov_id))
    }
}