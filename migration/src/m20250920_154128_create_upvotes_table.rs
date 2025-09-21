use crate::m20250914_122720_create_notes_table::Notes;
use sea_orm_migration::{prelude::*, schema::*};

const UQ_UPVOTES_NOTE_USER: &str = "uq_upvotes_note_user";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Upvotes::Table)
                    .if_not_exists()
                    .col(pk_auto(Upvotes::Id))
                    .col(integer(Upvotes::NoteId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_upvotes_notes_note-id")
                            .from(Upvotes::Table, Upvotes::NoteId)
                            .to(Notes::Table, Notes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .col(uuid(Upvotes::UserId))
                    .col(timestamp_with_time_zone(Upvotes::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .unique()
                    .name(UQ_UPVOTES_NOTE_USER)
                    .table(Upvotes::Table)
                    .col(Upvotes::NoteId)
                    .col(Upvotes::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(UQ_UPVOTES_NOTE_USER)
                    .table(Upvotes::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Upvotes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Upvotes {
    Table,
    Id,
    NoteId,
    UserId,
    CreatedAt,
}
