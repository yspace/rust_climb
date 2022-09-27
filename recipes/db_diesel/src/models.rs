use diesel::prelude::*;


#[derive(Queryable,Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published:  i32,
}

use crate::schema::posts;


#[derive(Insertable,Debug)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a i32,
}