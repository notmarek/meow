#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod models;
mod parsers;
use parsers::{parse_nyaa_list, parse_nyaa_torrent};
use rocket::http::uri::Origin;
use rocket::response::content;
use serde::Serialize;

fn structs_to_json<T>(data: Vec<T>) -> Vec<String>
where
    T: Serialize,
{
    let mut items: Vec<String> = Vec::new();
    for item in data {
        items.push(serde_json::to_string(&item).unwrap());
    }
    items
}

fn struct_to_json<T>(data: T) -> String
where
    T: Serialize,
{
    serde_json::to_string(&data).unwrap()
}

fn get_nyaa(uri: String) -> String {
    reqwest::blocking::ClientBuilder::new()
        .gzip(true)
        .build()
        .unwrap()
        .get(&uri)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

#[get("/nyaa")]
fn nyaa(uri: &Origin) -> content::Json<String> {
    let response = get_nyaa(format!(
        "https://nyaa.si/{}",
        uri.to_string().replace("/nyaa", "")
    ));
    content::Json(format!(
        "[{}]",
        structs_to_json(parse_nyaa_list(response, "https://nyaa.si")).join(", ")
    ))
}

#[get("/sukebei")]
fn sukebei(uri: &Origin) -> content::Json<String> {
    let response = get_nyaa(format!(
        "https://sukebei.nyaa.si/{}",
        uri.to_string().replace("/sukebei", "")
    ));
    content::Json(format!(
        "[{}]",
        structs_to_json(parse_nyaa_list(response, "https://sukebei.nyaa.si")).join(", ")
    ))
}

#[get("/nyaa/user/<user>")]
fn nyaauser(user: Option<String>) -> content::Json<String> {
    let response = get_nyaa(format!("https://nyaa.si/user/{}", user.unwrap()));
    content::Json(format!(
        "[{}]",
        structs_to_json(parse_nyaa_list(response, "https://nyaa.si")).join(", ")
    ))
}

#[get("/sukebei/user/<user>")]
fn sukebeiuser(user: Option<String>) -> content::Json<String> {
    let response = get_nyaa(format!("https://sukebei.nyaa.si/user/{}", user.unwrap()));
    content::Json(format!(
        "[{}]",
        structs_to_json(parse_nyaa_list(response, "https://sukebei.nyaa.si")).join(", ")
    ))
}

#[get("/nyaa/torrent/<id>")]
fn nyaatorrent(id: Option<String>) -> content::Json<String> {
    let response = get_nyaa(format!("https://nyaa.si/view/{}", id.unwrap()));
    
    content::Json(format!(
        "{}",
        struct_to_json(parse_nyaa_torrent(response, "https://nyaa.si"))
    ))
}

#[get("/sukebei/torrent/<id>")]
fn sukebeitorrent(id: Option<String>) -> content::Json<String> {
    let response = get_nyaa(format!("https://sukebei.nyaa.si/view/{}", id.unwrap()));
    
    content::Json(format!(
        "{}",
        struct_to_json(parse_nyaa_torrent(response, "https://sukebei.nyaa.si"))
    ))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![nyaa, nyaatorrent, nyaauser, sukebei, sukebeiuser, sukebeitorrent])
        .launch();
}
