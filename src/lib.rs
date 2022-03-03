#![warn(missing_docs)]
//! Downloads images from bing given certain days.

use reqwest::{blocking::Client, IntoUrl};

pub use chrono::prelude::*;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use image::{load_from_memory, DynamicImage, ImageFormat};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    images: Vec<Image>,
    tooltips: Tooltips,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    startdate: String,
    fullstartdate: String,
    enddate: String,
    url: String,
    urlbase: String,
    copyright: String,
    copyrightlink: String,
    title: String,
    quiz: String,
    wp: bool,
    hsh: String,
    drk: i64,
    top: i64,
    bot: i64,
    hs: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tooltips {
    loading: String,
    previous: String,
    next: String,
    walle: String,
    walls: String,
}

/// A generic error type encompassing serde_json and reqwest errors
#[derive(Debug)]
pub enum Error {
    /// A reqwest error
    Reqwest(reqwest::Error),
    /// A serde_json error
    Serde(serde_json::Error),
    /// An image error
    Image(image::ImageError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::Image(e)
    }
}

/// Downloads the images from the given dates
pub fn get_images(dates: &[NaiveDate]) -> Result<(), Error> {
    let idxs = get_idxs(dates);
    let client = Client::new();
    for idx in idxs {
        let image_url = get_image_url(idx, &client)?;
        let image = get_image_from_url(&image_url, &client)?;
        image.save_with_format(format!("{}.jpg", idx), ImageFormat::Jpeg)?;
    }
    Ok(())
}

fn get_idxs(dates: &[NaiveDate]) -> Vec<i64> {
    let today = Local::today().naive_local();
    let mut idxs = Vec::with_capacity(dates.len());
    for date in dates {
        idxs.push(today.signed_duration_since(date.clone()).num_days());
    }
    idxs
}

fn get_image_url(idx: i64, client: &Client) -> Result<String, Error> {
    let url = format!(
        "https://www.bing.com/HPImageArchive.aspx?format=js&idx={}&n=1",
        idx
    );
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        panic!("Received a server error: {:?}", resp.status());
    }
    let json: Root = resp.json()?;
    let mut url = String::from("https://bing.com");
    url.push_str(&json.images[0].url);
    Ok(url)
}

fn get_image_from_url<U: IntoUrl>(url: U, client: &Client) -> Result<DynamicImage, Error> {
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        panic!("Received a server error: {:?}", resp.status());
    }
    let bytes = resp.bytes()?;
    Ok(load_from_memory(&bytes)?)
}

#[test]
fn test_today_idx() {
    let dates = vec![Local::today().naive_local()];
    let idxs = get_idxs(&dates);
    assert_eq!(idxs, vec![0]);
}

#[test]
fn test_random_dates() {
    use chrono::Duration;
    use rand::prelude::*;
    const NUM_DAYS: usize = 100;
    let today = Local::today().naive_local();
    let mut nums = Vec::with_capacity(NUM_DAYS);
    let mut days = Vec::with_capacity(NUM_DAYS);
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_DAYS {
        let i = rng.gen_range(-500..0);
        nums.push(-i);
        days.push(today + Duration::days(i));
    }
    assert_eq!(nums, get_idxs(&days));
}

#[test]
fn test_url_aquisition() -> Result<(), Error> {
    let client = Client::new();
    let image_url = get_image_url(5, &client)?;
    assert_eq!(&image_url, "https://bing.com/th?id=OHR.LegacyMural_EN-US8368318184_1920x1080.jpg&rf=LaDigue_1920x1080.jpg&pid=hp");
    Ok(())
}
