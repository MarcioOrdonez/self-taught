use crate::diesel::{result::QueryResult};
use crate::{schema::subject, PgConnection};
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub creation_time: NaiveDateTime,
    pub deactivation_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "subject"]
pub struct NewSubject{
    pub title: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Default, Debug, Clone, Serialize, Deserialize)]
#[table_name = "subject"]
pub struct SubjectChangeSet {
    pub id: i32,
    pub title: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
}

impl NewSubject {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<Subject> {
        diesel::insert_into(subject::table).values(self).get_result(conn)
    }
}
