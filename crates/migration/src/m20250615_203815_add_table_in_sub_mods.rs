use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(SubMods::Table)
                    .add_column(
                        ColumnDef::new(SubMods::Subname)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
        .drop_table(Table::drop().table(SubMods::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
enum SubMods {
    Table,
    Subname
}
