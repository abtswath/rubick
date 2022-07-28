use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
    hash::Hasher,
    io::Write,
};

use anyhow::Result;
use async_std::prelude::StreamExt;
use reqwest::{Client, IntoUrl, Response, Url};
use tauri::utils::html;

use crate::application::image_dir;

const BASE_URL: &str = "https://www.douban.com";
const SEARCH: &str = "/search";
const DETAIL: &str = "/subject/";

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36";

struct DoubanError {
    error: Box<dyn Error + Send + Sync>,
}

impl Debug for DoubanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.error.to_string().as_str(), f)
    }
}

impl Display for DoubanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.error.to_string().as_str())
    }
}

impl DoubanError {
    pub fn new<E: Into<Box<dyn Error + Send + Sync>>>(error: E) -> Self {
        DoubanError {
            error: error.into(),
        }
    }
}

impl Error for DoubanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

pub struct Subject {
    pub pic: String,
    pub directors: String,
    pub writers: String,
    pub actors: String,
    pub types: String,
    pub released_at: String,
    pub summary: String,
    pub rating: f64,
}

async fn get<U: IntoUrl>(client: &Client, url: U) -> Result<Response> {
    let response = client
        .get(url)
        .header("User-Agent", DEFAULT_USER_AGENT)
        .send()
        .await?;
    Ok(response)
}

async fn get_subject_url(client: &Client, keyword: &str) -> Result<String> {
    let response = get(
        client,
        format!("{}{}?cat=1002&q={}", BASE_URL, SEARCH, keyword),
    )
    .await?;
    let contents = response.text().await?;
    let node = html::parse(contents);

    if let Ok(element) =
        node.select_first(".result-list>div.result:nth-of-type(1) .content .title a")
    {
        if let Some(href) = element.attributes.borrow().get("href") {
            let url = Url::parse(href)?;
            let mut pairs = url.query_pairs();
            let mut subject_url = String::from("");
            while let Some((key, value)) = pairs.next() {
                if key.eq("url") {
                    let url = Url::parse(&value)?;
                    if let Some(subject_id) = url.path().split("/").nth(2) {
                        subject_url = format!("{}{}{}", BASE_URL, DETAIL, subject_id);
                        break;
                    }
                }
            }
            return Ok(subject_url);
        }
    }

    Err(anyhow::Error::from(DoubanError::new(
        "Failed to get link of subject",
    )))
}

pub async fn get_subject(resource_name: &str) -> Result<Subject> {
    let client = Client::builder().build()?;
    let subject_url = get_subject_url(&client, resource_name).await?;
    let response = get(&client, subject_url).await?;
    let contents = response.text().await?;
    let mut subject = Subject {
        pic: String::from(""),
        directors: String::from(""),
        writers: String::from(""),
        actors: String::from(""),
        types: String::from(""),
        released_at: String::from(""),
        summary: String::from(""),
        rating: 0.0,
    };
    let document = html::parse(contents);
    if let Ok(element) = document.select_first("#content .article>div .subjectwrap") {
        let node = element.as_node();
        if let Ok(img) = node.select_first("#content .article>div .subjectwrap #mainpic>a>img") {
            if let Some(src) = img.attributes.borrow().get("src") {
                subject.pic = src.to_string();
            }
        }
        if let Ok(elements) = node.select(".subject #info>span:nth-child(1)>.attrs>a") {
            let mut directors = vec![];
            elements.for_each(|el| {
                directors.push(el.text_contents());
            });
            subject.directors = directors.join("/");
        }
        if let Ok(elements) = node.select(".subject #info>span:nth-child(3)>.attrs>a") {
            let mut writers = vec![];
            elements.for_each(|el| {
                writers.push(el.text_contents());
            });
            subject.writers = writers.join("/");
        }
        if let Ok(elements) = node.select(".subject #info>span:nth-child(5)>.attrs>a") {
            let mut actors = vec![];
            elements.for_each(|el| {
                actors.push(el.text_contents());
            });
            subject.actors = actors.join("/");
        }
        if let Ok(elements) = node.select(".subject #info>span[property='v:genre']") {
            let mut types = vec![];
            elements.for_each(|el| {
                types.push(el.text_contents());
            });
            subject.types = types.join("/");
        }
        if let Ok(el) = node.select_first(".subject #info>span[property='v:initialReleaseDate']") {
            subject.released_at = el.text_contents();
        }
        if let Ok(el) = node.select_first("#interest_sectl .rating_wrap .rating_self .rating_num") {
            if let Ok(rating) = el.text_contents().parse::<f64>() {
                subject.rating = rating;
            }
        }
    }
    if let Ok(element) = document.select_first(".related-info>div>span:nth-child(1)") {
        subject.summary = element.text_contents();
    }
    Ok(subject)
}

pub async fn download_image(src: &str) -> Result<String> {
    let mut hasher = DefaultHasher::new();
    hasher.write(src.as_bytes());
    let mut filename = hasher.finish().to_string();
    filename.push_str(".webp");

    let path = image_dir().join(&filename);

    let client = Client::builder().build()?;
    let response = get(&client, src).await?;
    let mut stream = response.bytes_stream();
    let mut file = File::create(path)?;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
    }
    Ok(filename)
}
