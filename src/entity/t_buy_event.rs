//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "t_buy_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account: Option<String>,
    pub buy_num: Option<i32>,
    pub bnbvalue: Option<String>,
    pub team: Option<i32>,
    pub rounds: Option<i32>,
    pub buyway: Option<i32>,
    pub invite_address: Option<String>,
    pub buy_time: Option<String>,
    pub state: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
