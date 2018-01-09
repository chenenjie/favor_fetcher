extern crate encoding;
#[macro_use]
extern crate error_chain;
extern crate file_fetcher;
extern crate regex;
extern crate reqwest;
extern crate victoria_dom;

// use std::fs::File;
// use std::env;
// use std::io::Write;
// use std::io::Read;
use reqwest::header::Headers;
use reqwest::{Method, Url};
use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};
use regex::Regex;

mod errors {
    error_chain!{
        types {
            ComicError, ErrorKind, ResultExt, ComicResult;
        }
        foreign_links{
            ReqwestError(::reqwest::Error);
        }
    }
}

fn main() {
    // let result = file_fetcher::open_bytes_str("https://crates.io/assets/Cargo-Logo-Small-c39abeb466d747f3be442698662c5260.png").unwrap();
    // let mut save_path = env::current_dir().unwrap();
    // save_path.push("fucking_good.png");

    // let mut file = File::create(save_path).unwrap();
    // file.write_all(&*result);

    // println!("{:?}", dir);
}

fn fetcher() -> errors::ComicError<String> {
    let site_path: Url = "http://m.lifanacg.com/shaonv/2016/0514/975.html"
        .parse()
        .unwrap();

    let tail = site_path
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_owned();

    println!("tail {}", tail);

    let re = Regex::new(r"共(?P<pagenum>\d{1,3})页").unwrap();

    //get page
    let page = send_request(site_path.clone()).and_then(|result| {
        if let Some(num) = re.captures(&result) {
            Ok(num["pagenum"].to_owned())
        } else {
            bail!("沒找到頁數");
        }
    })?;
}

fn send_request(url: Url) -> errors::ComicResult<String> {
    let mut headers = Headers::new();
    headers.set_raw("Content-Type", "text/html;charset=UTF-8");

    let client = reqwest::Client::new();
    let mut resp = client.request(Method::Get, url).headers(headers).send()?;

    println!("status {} ", resp.status().is_success());

    // let body = resp.text().unwrap();
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();

    let body = GBK.decode(&*buf, DecoderTrap::Replace).unwrap();

    Ok(body)
}
