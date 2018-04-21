#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate maud;
extern crate serde_json;

mod skill;
mod work;

use std::fs::File;
use std::io::prelude::*;
use maud::DOCTYPE;
use maud::html;
use skill::Skills;
use work::Works;

fn read_file(filename: String) -> String {
    let mut file = File::open(filename).expect("not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("");

    return content;
}

fn main() {
    let skills_json= read_file("./src/skills.json".to_string());
    let skills: Skills = serde_json::from_str(&skills_json).unwrap();
    let works_json= read_file("./src/works.json".to_string());
    let works: Works = serde_json::from_str(&works_json).unwrap();
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
                div class="container text" {
                    h1 class="title" "Profile of Yoshihiro Tanaka"
                    div class="top" {
                        "Yoshihiro Tanaka"
                        br;
                        "contact@cordea.jp"
                    }
                    div {
                        span class="top-section" {
                            "Abstract. "
                        }
                        "Abstract"
                    }
                    div {
                        p class="section" {
                            "1. Introduction"
                        }
                        p {
                            (introduction)
                        }
                    }
                    div {
                        p class="section" {
                            "2. Skills"
                        }
                        p {
                            "test"
                            table {
                                tr class="header" {
                                    th class="language" "Language"
                                    th "Work experience"
                                    th "Hobby"
                                    th "Description"
                                }
                                @for skill in skills.skills {
                                    @if skill.main {
                                    tr {
                                        td (skill.title)
                                        td class="experience" (skill.work)
                                        td class="experience" (skill.hobby)
                                        td class="description" (skill.description)
                                    }
                                }
                            }
                            }
                        }
                    }
                    div {
                        p class="section" {
                            "3. Works"
                        }
                        p {
                            @for work in works.works {
                                li {
                                    b {
                                        (work.name)
                                    }
                                    " - " (work.period)
                                }
                                p (work.content)
                                p (work.description)
                            }
                        }
                    }
                    div {
                        p class="section" {
                            "5. Conclusions"
                        }
                        p {
                            ""
                        }
                    }
                    div {
                        p class="section" {
                            "Acknowledgments"
                        }
                        p {
                            "Thank you for reading."
                        }
                    }
                    div {
                        p class="section" {
                            "References"
                        }
                        p {
                            ""
                        }
                    }
                }
            }
        }
    };
    println!("{}", html.into_string());
}
