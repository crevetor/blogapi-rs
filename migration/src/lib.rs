#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20240103_033835_tags;
mod m20240103_034724_posts;
mod m20240103_035039_posts_tags;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240103_033835_tags::Migration),
            Box::new(m20240103_034724_posts::Migration),
            Box::new(m20240103_035039_posts_tags::Migration),
        ]
    }
}
