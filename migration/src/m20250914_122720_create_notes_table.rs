use crate::extension::postgres::Type;
use crate::m20250907_130748_create_rooms_table::Rooms;
use crate::sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
struct BoardEnum;

#[derive(DeriveIden, EnumIter)]
enum BoardEnumVariants {
    #[sea_orm(iden = "STRAW")]
    Straw,

    #[sea_orm(iden = "STICKS")]
    Sticks,

    #[sea_orm(iden = "BRICKS")]
    Bricks,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(BoardEnum)
                    .values(BoardEnumVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(pk_auto(Notes::Id))
                    .col(integer(Notes::RoomId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_notes_rooms_room-id")
                            .from(Notes::Table, Notes::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .col(enumeration(
                        Notes::Board,
                        BoardEnum,
                        BoardEnumVariants::iter(),
                    ))
                    .col(text(Notes::Description))
                    .col(
                        timestamp_with_time_zone(Notes::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Notes::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Notes::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notes::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(BoardEnum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Notes {
    Table,
    Id,
    RoomId,
    Board,
    Description,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
