pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250613_175020_add_new_tables;
mod m20250615_203815_add_table_in_sub_mods;
mod m20250620_210632_add_table_in_user_joined_subs;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250613_175020_add_new_tables::Migration),
            Box::new(m20250615_203815_add_table_in_sub_mods::Migration),
            Box::new(m20250620_210632_add_table_in_user_joined_subs::Migration),
        ]
    }
}
