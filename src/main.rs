mod database;
mod error;

#[macro_use]
extern crate rocket;

pub use self::error::{Error, Result};

use crate::database::Member;

use rocket::response::status::Created;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::Acquire;

#[derive(Database)]
#[database("sqlite_badger")]
pub struct BadgerDB(sqlx::SqlitePool);

#[get("/member/<fob_id>")]
async fn get_member(db: Connection<BadgerDB>, fob_id: &str) -> Result<Json<Member>> {
    let member = database::get_member_by_id(db, fob_id.into()).await?;

    Ok(Json(member))
}

#[post("/member", data = "<member>")]
async fn create_member(
    db: Connection<BadgerDB>,
    member: Json<Member>,
) -> Result<Created<Json<Member>>> {
    let result = database::create_member(db, member.0).await?;

    Ok(Created::new("/member").body(Json(result)))
}

#[put("/member/<fob_id>", data = "<member_data>")]
async fn update_member(
    db: Connection<BadgerDB>,
    fob_id: &str,
    member_data: Json<Member>,
) -> Result<Created<Json<Member>>> {
    let member = member_data.0;
    if member.fob_id != fob_id {
        return Err(Error::BadRequest("invalid fob_id in member data".into()));
    }
    let result = database::update_member(db, member).await?;

    Ok(Created::new("/member").body(Json(result)))
}

#[catch(404)]
fn general_not_found() -> Value {
    json!({
        "code": 404,
        "status": "error",
               "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(BadgerDB::init())
        .mount("/api/v1", routes![get_member, create_member, update_member])
}
