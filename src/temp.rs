use chrono::{Datelike, DateTime, Local, Utc};
use comrak::{ComrakExtensionOptions, ComrakOptions, markdown_to_html};
use glob::glob;
use serde::Serialize;

use crate::projects::Projects;
use crate::skills::Skills;

#[derive(Serialize)]
pub struct Index {
    name: String,
    title: String,
    year: String,
    version: String,
    projects: Vec<Projects>,
    skill: Vec<Skills>,
}

#[derive(Serialize)]
pub struct Post {
    release_date: String,
    blog_title: String,
    blog_link: String,
}

#[derive(Serialize)]
pub struct BlogIndex {
    title: String,
    posts: Vec<Post>,
    year: String,
    version: String,
}

#[derive(Serialize)]
pub struct PostIndex {
    title: String,
    post: String,
    year: String,
    version: String,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            name: "Karthikey Hegde".to_string(),
            title: "Karthikey's Portfolio".to_string(),
            year: Local::now().date().year().to_string(),
            version: rustc_version_runtime::version().to_string(),
            projects: Projects::default(),
            skill: Skills::default(),
        }
    }
}

impl Post {
    fn get_posts() -> Vec<Self> {
        let file_list = glob("articles/*.md").unwrap();
        let posts: Vec<Post> = file_list
            .map(|f| {
                let fname = f.as_ref().unwrap().display().to_string();
                let split = fname.splitn(2, '/').collect::<Vec<&str>>()[1];
                Post {
                    release_date: DateTime::<Utc>::from(
                        f.as_ref().unwrap().metadata().unwrap().modified().unwrap(),
                    )
                        .format("%d/%b/%Y")
                        .to_string(),
                    blog_link: split.replace(".md", ""),
                    blog_title: split.replace(".md", "").replace("_", " "),
                }
            })
            .collect();
        posts
    }
}

impl Default for BlogIndex {
    fn default() -> Self {
        Self {
            title: "Blog".to_string(),
            posts: Post::get_posts(),
            year: Local::now().date().year().to_string(),
            version: rustc_version_runtime::version().to_string(),
        }
    }
}

impl PostIndex {
    pub fn new(file: String) -> Self {
        let md_text = std::fs::read_to_string(&file).unwrap();
        let opt = ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: false,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: true,
                header_ids: None,
                footnotes: false,
                description_lists: false,
                front_matter_delimiter: None,
            },
            ..Default::default()
        };
        let post_html = markdown_to_html(&md_text, &opt);
        Self {
            title: file
                .split('/')
                .last()
                .unwrap()
                .replace(".md", "")
                .replace("_", " "),
            post: post_html,
            year: Local::now().date().year().to_string(),
            version: rustc_version_runtime::version().to_string(),
        }
    }
}
