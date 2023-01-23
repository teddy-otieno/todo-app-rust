use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::todo;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name=todo)]
pub struct Todo {
    pub id: i32,
    pub item: String,
    pub done: bool
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name=todo)]
pub struct NewTodo {
    pub item: String,
    pub done: bool
}


#[derive(AsChangeset, Identifiable, Clone, Copy)]
#[diesel(table_name=todo)]
pub struct MarkDoneChange {
    pub id: i32,
    pub done: bool, 
}
