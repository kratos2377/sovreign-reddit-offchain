use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts


          manager
            .create_table(
                Table::create()
                    .table(UserJoinedSubs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserJoinedSubs::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserJoinedSubs::User_Sov_Id).string().not_null())
                    .col(ColumnDef::new(UserJoinedSubs::Sub_Sov_Id).string().not_null())
                                   .col(ColumnDef::new(UserJoinedSubs::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(UserJoinedSubs::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_joined_subs_user_sov_id")
                            .from(UserJoinedSubs::Table, UserJoinedSubs::User_Sov_Id)
                            .to(Users::Table, Users::Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_joined_subs_sub_sov_id")
                            .from(UserJoinedSubs::Table, UserJoinedSubs::Sub_Sov_Id)
                            .to(Subreddit::Table, Subreddit::Sub_Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;



                  manager
            .create_table(
                Table::create()
                    .table(UserLikedPosts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLikedPosts::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserLikedPosts::Value).integer().not_null()) // 1 for upvote, -1 for downvote
                    .col(ColumnDef::new(UserLikedPosts::User_Sov_Id).string().not_null())
                    .col(ColumnDef::new(UserLikedPosts::Post_Sov_Id).string().not_null())
                                   .col(ColumnDef::new(UserLikedPosts::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(UserLikedPosts::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_liked_posts_user_sov_id")
                            .from(UserLikedPosts::Table, UserLikedPosts::User_Sov_Id)
                            .to(Users::Table, Users::Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_liked_posts_post_sov_id")
                            .from(UserLikedPosts::Table, UserLikedPosts::Post_Sov_Id)
                            .to(Posts::Table, Posts::Post_Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;


             manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_joined_subs_user_sov_id")
                    .table(UserJoinedSubs::Table)
                    .col(UserJoinedSubs::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_joined_subs_sub_sov_id")
                    .table(UserJoinedSubs::Table)
                    .col(UserJoinedSubs::Sub_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Unique constraint for user-subreddit combination
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_joined_subs_unique")
                    .table(UserJoinedSubs::Table)
                    .col(UserJoinedSubs::User_Sov_Id)
                    .col(UserJoinedSubs::Sub_Sov_Id)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // User liked posts indexes
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_liked_posts_user_sov_id")
                    .table(UserLikedPosts::Table)
                    .col(UserLikedPosts::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_liked_posts_post_sov_id")
                    .table(UserLikedPosts::Table)
                    .col(UserLikedPosts::Post_Sov_Id)
                    .to_owned(),
            )
            .await?;


            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
   manager
            .drop_table(Table::drop().table(UserLikedPosts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserJoinedSubs::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserJoinedSubs {
    Table,
    Id,
    Sub_Sov_Id,
    User_Sov_Id,
        CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserLikedPosts {
    Table,
    Id,
    Post_Sov_Id,
    User_Sov_Id,
    Value,
        CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Post_Sov_Id,
    Title,
    Content,
    Sub_Sov_Id,
    Flair,
    User_Sov_Id,
    Upvote,
    Downvote,
    Score,
        CreatedAt,
    UpdatedAt,
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Sov_Id,
    Username,
        CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Subreddit {
    Table,
    Sub_Sov_Id,
    Subname,
    Sub_Description,
        CreatedAt,
    UpdatedAt,
}