use diesel::prelude::*;



#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name= crate::schema::users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String
}

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub name: String
}