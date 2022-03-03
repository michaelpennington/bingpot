use bingpot::{get_images, Error};
use chrono::prelude::*;

fn main() -> Result<(), Error> {
    let mut days = Vec::new();
    for i in 8..16 {
        let today = Local::today().naive_local();
        days.push(today + chrono::Duration::days(-i));
    }
    get_images(&days)?;
    Ok(())
}
