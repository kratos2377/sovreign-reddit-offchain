use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Sov_Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string())
                    .to_owned(),
            )
            .await?;

        // Create sub table
        manager
            .create_table(
                Table::create()
                    .table(Subreddit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subreddit::Sub_Sov_Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subreddit::Subname).string())
                    .col(ColumnDef::new(Subreddit::Sub_Description).string())
                    .to_owned(),
            )
            .await?;

        // Create posts table
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Post_Sov_Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::Title).string())
                    .col(ColumnDef::new(Posts::Content).string())
                    .col(ColumnDef::new(Posts::Sub_Sov_Id).string())
                    .col(ColumnDef::new(Posts::Flair).string())
                    .col(ColumnDef::new(Posts::User_Sov_Id).string())
                    .col(ColumnDef::new(Posts::Upvote).integer())
                    .col(ColumnDef::new(Posts::Downvote).integer())
                    .col(ColumnDef::new(Posts::Score).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_user_sov_id")
                            .from(Posts::Table, Posts::User_Sov_Id)
                            .to(Users::Table, Users::Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_sub_sov_id")
                            .from(Posts::Table, Posts::Sub_Sov_Id)
                            .to(Subreddit::Table, Subreddit::Sub_Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create user_joined_subs table
        manager
            .create_table(
                Table::create()
                    .table(UserJoinedSubs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserJoinedSubs::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserJoinedSubs::User_Sov_Id).string())
                    .col(ColumnDef::new(UserJoinedSubs::Sub_Sov_Id).string())
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

        // Create comments table
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::User_Sov_Id).string())
                    .col(ColumnDef::new(Comments::Post_Sov_Id).string())
                    .col(ColumnDef::new(Comments::Content).string())
                    .col(ColumnDef::new(Comments::Upvote).integer())
                    .col(ColumnDef::new(Comments::Downvote).integer())
                    .col(ColumnDef::new(Comments::Score).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_user_sov_id")
                            .from(Comments::Table, Comments::User_Sov_Id)
                            .to(Users::Table, Users::Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_post_sov_id")
                            .from(Comments::Table, Comments::Post_Sov_Id)
                            .to(Posts::Table, Posts::Post_Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;


               // Create sub_mods table
        manager
            .create_table(
                Table::create()
                    .table(SubMods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubMods::Sub_Sov_Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SubMods::User_Sov_Id).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sub_mods_user_sov_id")
                            .from(SubMods::Table, SubMods::User_Sov_Id)
                            .to(Users::Table, Users::Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sub_mods_sub_sov_id")
                            .from(SubMods::Table, SubMods::Sub_Sov_Id)
                            .to(Subreddit::Table, Subreddit::Sub_Sov_Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;



        // Create indexes for all *sov_id columns for fast searching
        
        // Index for user_joined_subs.user_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_user_joined_subs_user_sov_id")
                    .table(UserJoinedSubs::Table)
                    .col(UserJoinedSubs::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Index for user_joined_subs.sub_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_user_joined_subs_sub_sov_id")
                    .table(UserJoinedSubs::Table)
                    .col(UserJoinedSubs::Sub_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Index for posts.user_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_user_sov_id")
                    .table(Posts::Table)
                    .col(Posts::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Index for posts.sub_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_posts_sub_sov_id")
                    .table(Posts::Table)
                    .col(Posts::Sub_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Index for comments.user_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_comments_user_sov_id")
                    .table(Comments::Table)
                    .col(Comments::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        // Index for comments.post_sov_id
        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_sov_id")
                    .table(Comments::Table)
                    .col(Comments::Post_Sov_Id)
                    .to_owned(),
            )
            .await?;


         manager
            .create_index(
                Index::create()
                    .name("idx_sub_mods_user_sov_id")
                    .table(SubMods::Table)
                    .col(SubMods::User_Sov_Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to handle foreign key constraints

           manager
            .drop_table(Table::drop().table(SubMods::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserJoinedSubs::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Subreddit::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Sov_Id,
    Username,
}

#[derive(DeriveIden)]
enum Subreddit {
    Table,
    Sub_Sov_Id,
    Subname,
    Sub_Description
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
}

#[derive(DeriveIden)]
enum UserJoinedSubs {
    Table,
    Id,
    User_Sov_Id,
    Sub_Sov_Id,
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    User_Sov_Id,
    Post_Sov_Id,
    Content,
    Upvote,
    Downvote,
    Score,
}

#[derive(Iden)]
enum SubMods {
    Table,
    Sub_Sov_Id,
    User_Sov_Id,
}