use sea_orm::DeriveEntityModel;
use sea_orm::prelude::DateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub repo_path: String,
    pub repo_url: String,
    pub disable: bool,
    pub create_time: DateTime,
    pub update_time: DateTime,
    pub create_by: String,
    pub update_by: String,
}
