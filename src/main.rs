extern crate reqwest;
use rand::Rng;
use serde_json::Value;
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
    println!("Explanation: https://www.explainxkcd.com/wiki/index.php/{}",number);
    Ok(()) // input variable

    // parse into generic JSON value
}
