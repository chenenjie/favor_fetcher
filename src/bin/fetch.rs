extern crate file_fetcher;
extern crate reqwest;
extern crate victoria_dom;

// use std::fs::File;
// use std::env;
// use std::io::Write;
// use std::io::Read;
use reqwest::header::Headers;
use reqwest::Method;

fn main() {
    // let result = file_fetcher::open_bytes_str("https://crates.io/assets/Cargo-Logo-Small-c39abeb466d747f3be442698662c5260.png").unwrap();
    // let mut save_path = env::current_dir().unwrap();
    // save_path.push("fucking_good.png");

    // let mut file = File::create(save_path).unwrap();
    // file.write_all(&*result);

    // let mut resp = reqwest::get("http://m.lifanacg.com/shaonv/2016/0514/975.html").unwrap();

    let mut headers = Headers::new();
    headers.set_raw("Content-Type", "text/html;charset=UTF-8");

    let client = reqwest::Client::new();
    let mut resp = client
        .request(
            Method::Get,
            "http://m.lifanacg.com/shaonv/2016/0514/975.html",
        )
        .headers(headers)
        .send()
        .unwrap();

    println!("status {} ", resp.status().is_success());

    // let body = resp.text().unwrap();
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();

    println!("body = {:?}", buf);

    // println!("{:?}", dir);
}
