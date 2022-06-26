use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

pub type Amount = u64;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Participant {
    pub user_uuid: Uuid,
    pub amount: Amount,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Expense {
    pub uuid: Uuid,
    pub title: String,
    pub date: DateTime,
    pub image: String,
    pub payed_amount: Amount,
    pub payer_uuid: Uuid,
    pub participants: Vec<Participant>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Group {
    pub uuid: Uuid,
    pub title: String,
    pub description: String,
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
}
