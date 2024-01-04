extern crate reqwest;
use rand::Rng;
use serde_json::Value;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen();
    number.to_string();
    let res: Value = client.get("https://xkcd.com/info.0.json").send()?.json()?;
    //println!("{}", res);
    let img_link = res["img"].as_str().unwrap();
    let alt_text = res["alt"].as_str().unwrap();
    println!("{:?}", img_link);
    println!("{:?}", alt_text);
    Ok(()) // input variable

    // parse into generic JSON value
}
