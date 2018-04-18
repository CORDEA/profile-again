#![feature(proc_macro)]

extern crate maud;
use maud::DOCTYPE;
use maud::html;

fn main() {
    let html = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8"
                meta name="viewport"  content="width=device-width, initial-scale=1.0"

                link href="./src/main.css" rel="stylesheet"
                title {
                    "title"
                }
            }
            body {
                div class="container" {
                    h1 "title"
                    div {
                        p {
                            "Abstract"
                        }
                    }
                    div {
                        h2 {
                            "1. Introduction"
                        }
                        p {
                            "test"
                        }
                    }
                }
            }
        }
    };
    println!("{}", html.into_string());
}
