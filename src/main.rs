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
    let introduction = "I am an android applications engineer at the CyberAgent Inc, and live in Tokyo. CyberAgent is my second company and I have been working since Sep 2017. I currently focus on development of Android native applications only, but always have interests widely, and I am still working on many things as a hobby. For example, Flutter, AWS, Docker, Font-end technologies etc. In programming language, I mainly use Java / Kotlin at work. As a hobby, developing using many programming languages, using Python, Nim, Golang, C#, JavaScript etc (Please see Skills for details). Native language is Japanese. English uses to read api document and to write commit message, issue, blog, but I am not good at talking.";
    let sub_langs = skills.skills
        .iter()
        .filter(|s| !s.main)
        .map(|s| s.title.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let html = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8"
                meta name="viewport"  content="width=device-width, initial-scale=1.0"

                link href="./src/main.css" rel="stylesheet"
                title "Yoshihiro Tanaka's Profile"
            }
            body {
                div class="container text" {
                    h1 class="title" "Yoshihiro Tanaka's Profile"
                    div class="top" {
                        "Yoshihiro Tanaka"
                        br;
                        "contact@cordea.jp"
                    }
                    div {
                        span class="top-section" "Abstract. "
                        "Android applications engineer at the CyberAgent Inc. I love good codes, good tests, good team, respectable developers."
                    }
                    div {
                        p class="section" "1. Introduction"
                        p (introduction)
                    }
                    div {
                        p class="section" "2. Skills"
                        p "I mainly use the following programming languages."
                        p class="table-title" "Table 1. Programming languages list that I mainly use."
                        table {
                            tr class="header" {
                                th class="language" "Programming language"
                                th "Work experience (years)"
                                th "Hobby (years)"
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
                        p {
                            "I also use many other programming languages: "
                            (sub_langs)
                            "."
                        }
                    }
                }
                div class="container text" {
                    div {
                        p class="section" "3. Works"
                        p {
                            @for work in works.works {
                                li {
                                    b (work.name)
                                    " - " (work.period)
                                }
                                p (work.content)
                                p (work.description)
                            }
                        }
                    }
                    div {
                        p class="section" "4. Changing jobs"
                        p "Recently, I have receiving so many e-mails and linkedin's requests about changing jobs. I am reading most messages, and thanks for senders. However, my time isn't infinite, so it's difficult to replay to all."
                    }
                    div {
                        p class="section" "5. Conclusions"
                        p "Please look at my social accounts if you want to know me more. For example, GitHub [1], Hatena Blog [2], Twitter [3], Qiita [4]."
                    }
                    div {
                        p class="section" "Acknowledgments"
                        p "Thank you for reading."
                    }
                    div {
                        p class="section" "References"
                        p {
                            a href="https://github.com/CORDEA" "[1] Yoshihiro Tanaka. CORDEA (Yoshihiro Tanaka). https://github.com/CORDEA."
                            br;
                            a href="http://cordea.hatenadiary.com/" "[2] CORDEA. CORDEA blog. http://cordea.hatenadiary.com/."
                            br;
                            a href="https://twitter.com/_Cordea" "[3] Yoshihiro Tanaka. Yoshihiro Tanaka (@_Cordea) | Twitter. https://twitter.com/_Cordea."
                            br;
                            a href="https://qiita.com/CORDEA" "[4] CORDEA. CORDEA - Qiita. https://qiita.com/CORDEA."
                        }
                    }
                }
            }
        }
    };
    println!("{}", html.into_string());
}
