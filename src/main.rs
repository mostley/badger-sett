#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Member {
    contents: &str,
}

#[get("/member/<fobId>")]
fn getMember(fobId: &str) -> Json<Member> {
    format!("Hello, {} year old named {}!", fobId)
}

#[post("/member", data = "<member>")]
fn createMember(name: &str, member: Form<Member<'_>>) -> String {
    format!("Hello, {} year old named {}!",)
}

#[put("/member/<fobId>", data = "<msg>")]
fn updateMember(fobId: &str, msg: Json<Message<'_>>) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![getMember, createMember, updaterMember])
}
