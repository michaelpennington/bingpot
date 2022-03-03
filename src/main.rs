use reqwest::{blocking::Client, IntoUrl};

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

#[derive(Debug)]
enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
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

fn get_bing_iotd() -> Result<(), Error> {
    let client = Client::new();
    let image_url = get_image_url(&client)?;
    let image = get_image_from_url(&image_url, &client)?;
    image.save_with_format("wallpaper.jpg", ImageFormat::Jpeg)?;
    Ok(())
}

fn get_image_url(client: &Client) -> Result<String, Error> {
    let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1";
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

fn main() -> Result<(), Error> {
    get_bing_iotd()
}
