/*
rocket web 服务
*/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

#[get("/users")]
fn users() -> &'static str {
    "{\"users\": [{\"id\": 1, \"name\": \"John Doe\"}, {\"id\": 2, \"name\": \"Jane Doe\"}]}"
}

fn main() {
    rocket::ignite().mount("/", routes![users]).launch();
}