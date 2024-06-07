#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection, Database};

#[derive(Serialize, Deserialize)]
struct Member {
    fob_id: String,
    name: String,
    contact_data: String,
}

#[derive(Database)]
#[database("sqlite_badger")]
struct Badger(sqlx::SqlitePool);

async fn get_member_by_id(mut db: Connection<Badger>, tag_number: Vec<u8>) -> Option<Json<Member>> {
    sqlx::query!(
        "SELECT Tag, Name, Comment FROM Tags WHERE Tag = ?",
        tag_number
    )
    .fetch_one(&mut **db)
    .await
    .and_then(|r| {
        Ok(Json(Member {
            fob_id: hex::encode(tag_number),
            name: r.Name.unwrap(),
            contact_data: r.Comment.unwrap(),
        }))
    })
    .ok()
}

#[get("/member/<fob_id>")]
async fn get_member(db: Connection<Badger>, fob_id: &str) -> Option<Json<Member>> {
    let tag_number = hex::decode(fob_id);
    match tag_number {
        Ok(tag_number) => get_member_by_id(db, tag_number).await,
        Err(err) => {
            print!("{:?}", err);

            // invalid fob_id should lead to 404
            None
        }
    }
}

#[post("/member", data = "<member>")]
fn create_member(member: Json<Member>) -> Json<Member> {
    member
}

#[put("/member/<fob_id>", data = "<member_data>")]
fn update_member(fob_id: &str, member_data: Json<Member>) -> Option<Json<Member>> {
    Some(Json(Member {
        fob_id: fob_id.into(),
        name: "pete".into(),
        contact_data: "somewhere".into(),
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Badger::init())
        .mount("/api/v1", routes![get_member, create_member, update_member])
}
