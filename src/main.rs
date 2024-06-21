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
            class: "pt-20 bg-black text-white grid lg:grid-cols-2 px-14 lg:px-64",
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
            h2{class:"text-xl","CSE Student"}
            h3{"I build things"}
            div{"social"}
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        h2{class: "","ABOUT"}
        p{"My journey into the world of technology began with a profound passion for Rust and its application in OS development. Although I've delved into building tiny operating systems, my true enthusiasm lies in utilizing Rust's power to solve complex problems and build efficient, high-performance software."}
        p{"As a Rust enthusiast, I enjoy exploring how this language can revolutionize everything from system-level programming to web applications. My work often involves integrating Rust’s safety and concurrency features to create robust and scalable solutions."}
        p{"I thrive on the challenge of finding innovative ways to merge Rust’s strengths with other technologies to tackle diverse technical problems. Whether optimizing backend architectures or crafting sleek interfaces, I aim to deliver solutions that are both elegant and effective. When I'm not deep into coding, you can find me exploring cybersecurity or keeping up with the latest tech advancements."}

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
