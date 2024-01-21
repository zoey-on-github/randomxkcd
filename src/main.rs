extern crate reqwest;
use image;
use rand::Rng;
use serde_json::Value;
use viuer::Config;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen_range(1..2878);
    number.to_string();
    let url = format!("https://xkcd.com/{}/info.0.json", number);
    let res: Value = client.get(url).send()?.json()?;
    println!("Comic #{}", number);
    //println!("{}", res);
    let img_link = res["img"].as_str().unwrap();
    let alt_text = res["alt"].as_str().unwrap();
    println!("{:?}", img_link);
    println!("{:?}", alt_text);
    println!(
        "Explanation: https://www.explainxkcd.com/wiki/index.php/{}",
        number
    );
    let img_bytes = reqwest::blocking::get(img_link)?.bytes()?;
    let image = image::load_from_memory(&img_bytes)?;
    let conf = Config {
        // set offset
        x: 20,
        y: 4,
        // set dimensions
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };
    viuer::print(&image, &conf).expect("Image printing failed.");
    Ok(()) // input variable

    // parse into generic JSON value
}
