use crate::{
    schema::{check_point},
    PgConnection,
};
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckPoint {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub creation_time: NaiveDateTime,
    pub deactivation_time: Option<NaiveDateTime>,
    pub precedence: i32,
    pub subject_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "check_point"]
pub struct NewCheckPoint {
    pub title: String,
    pub subject_id: i32,
    pub description: Option<String>,
    pub precedence: Option<i32>,
}

#[derive(AsChangeset, Default, Debug, Clone, Serialize, Deserialize)]
#[table_name = "check_point"]
pub struct CheckPointChangeSet {
    pub id: i32,
    pub title: Option<String>,
    pub subject_id: Option<i32>,
    pub precedence: Option<i32>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub deactivation_time: Option<NaiveDateTime>,
}

impl NewCheckPoint {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<CheckPoint> {
        self.insert_into(check_point::table).get_result(conn)
    }
}
