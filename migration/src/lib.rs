pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250907_130748_create_rooms_table;
mod m20250914_122720_create_notes_table;
mod m20250920_133412_add_user_id_to_notes;
mod m20250920_154128_create_upvotes_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250907_130748_create_rooms_table::Migration),
            Box::new(m20250914_122720_create_notes_table::Migration),
            Box::new(m20250920_133412_add_user_id_to_notes::Migration),
            Box::new(m20250920_154128_create_upvotes_table::Migration),
        ]
    }
}
