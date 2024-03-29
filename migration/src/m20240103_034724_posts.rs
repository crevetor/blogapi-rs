use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Posts::Table)
                    .col(pk_auto(Posts::Id).borrow_mut())
                    .col(string(Posts::Title).borrow_mut())
                    .col(text(Posts::Summary).borrow_mut())
                    .col(text(Posts::Content).borrow_mut())
                    .col(timestamp_null(Posts::PublishedDate).borrow_mut())
                    .col(integer(Posts::UserId).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts-users")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Summary,
    Content,
    PublishedDate,
    UserId,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
