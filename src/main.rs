#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "bg-black text-white grid lg:grid-cols-2 px-12",
            Intro{}
            div{
                About{}
                Experience{}
                Projects{}
                Writing{}
            }
        }
    }
}

#[component]
fn Intro() -> Element {
    rsx! {
        div {
            h1{"Thej Zain"}
            h2{"CSE Student"}
            h3{"lines"}
            div{"social"}
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        h2{"ABOUT"}
        h3{"About lines 3 para"}
    }
}

#[component]
fn Experience() -> Element {
    rsx! {
        h2{"EXPERIENCE"}
        div{
            class : "grid grid-cols-3",
            div{"2024-PRESENT"}
            div{
                class:"col-span-2",
                div{
                    class:"grid grid-rows-3",
                    div{"Name"}
                    div{"details"}
                    div{"tech"}
                }
            }
        }
        div{"View Full Resume"}
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        h2{"PROJECTS"}
        div{
            class : "grid grid-cols-3",
            div{"pic"}
            div{
                class:"col-span-2",
                div{
                    div{"Name"}
                    div{"desc"}
                }
            }
        }
        div{"View Full Project Archive"}
    }
}

#[component]
fn Writing() -> Element {
    rsx! {
        h2{"WRITING"}
        div{
            class : "grid grid-cols-3",
            div{"pic"}
            div{class:"col-span-2", div{ div{"Date"} div{"Name"}}}
        }
    }
}
#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
