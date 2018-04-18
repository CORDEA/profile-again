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
                title {
                    "title"
                }
            }
            body {
                div {
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
