#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Member {
    fob_id: String,
    name: String,
    contact_data: String,
}

#[get("/member/<fob_id>")]
fn get_member(fob_id: &str) -> Option<Json<Member>> {
    Some(Json(Member {
        fob_id: fob_id.into(),
        name: "pete".into(),
        contact_data: "somewhere".into(),
    }))
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
    rocket::build().mount("/api/v1", routes![get_member, create_member, update_member])
}
