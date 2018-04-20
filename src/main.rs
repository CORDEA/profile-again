#![feature(proc_macro)]

extern crate maud;
use maud::DOCTYPE;
use maud::html;

fn main() {
    let introduction = "I am an android applications engineer at the CyberAgent Inc, and live in Tokyo. CyberAgent is my second company and I have been working since Sep 2017. I mainly use Java / Kotlin at work. As a hobby, developing using many languages, using Python, Nim, Golang, C#, JavaScript etc (Please see Skills for details). Native language is Japanese. English uses to read api document and to write commit message, issue, blog, but I am not good at talking.";

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
