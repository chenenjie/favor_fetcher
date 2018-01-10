extern crate encoding;
#[macro_use]
extern crate error_chain;
extern crate file_fetcher;
extern crate regex;
extern crate reqwest;
extern crate victoria_dom;

use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use std::env;
use std::io::Write;
use std::io::Read;
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
            ParseIntError(::std::num::ParseIntError);
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
    println!("{:?}", fetcher());
}

fn fetcher() -> errors::ComicResult<()> {
    let site_path: Url = "http://m.lifanacg.com/shaonv/2016/0514/975.html"
        .parse()
        .unwrap();

    let tail = site_path
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_owned()
        .replace(".html", "");

    let re = Regex::new(r"共(?P<pagenum>\d{1,3})页").unwrap();
    let title_re = Regex::new(r"<title>(?P<title>.*)</title>").unwrap();
    let re1 = Regex::new("<img .*src=\"(?P<img>.*)\"/>").unwrap();

    //get page
    let (page, title): (i32, String) = send_request(site_path.clone()).and_then(|result| {
        let title = match title_re.captures(&result) {
            Some(cap) => cap["title"].to_owned(),
            None => bail!("没有找到标题"),
        };
        let pagnum = match re.captures(&result) {
            Some(cap) => cap["pagenum"].to_owned(),
            None => bail!("没有找到页数"),
        };
        Ok((pagnum.parse()?, title))
    })?;

    println!("page {}, title {}", page, title);

    //创建存放的文件夹
    let book_dir = creater_dir(&title);

    let mut fetch_url_list: Vec<(i32, Url)> = {
        (2..(page + 1))
            .map(|i| {
                let mut single_page = site_path.clone();
                single_page
                    .path_segments_mut()
                    .unwrap()
                    .pop()
                    .push(&format!("{}_{}.html", tail, i));
                (i, single_page)
            })
            .collect()
    };

    let (succ_vec, fail_vec) : (Vec<(i32, Url, errors::ComicResult<String>)>, Vec<(i32, Url, errors::ComicResult<String>)>) = fetch_url_list.into_iter().map(|(num, single_page)|{
        let url_entity = send_request(single_page.clone()).and_then(|result| {

            let cap = re1.captures(&result).unwrap();

            Ok(cap["img"].to_owned())
        });
        (num, single_page, url_entity)
    }).partition(|&(_num, _single_page, url_entity)|{
        url_entity.is_ok()
    });

    println!("success vec {:?}", succ_vec);
    println!("fail vec{:?}", fail_vec);

    // println!("img_url {}", img_url.unwrap());

    Ok(())
}

fn creater_dir(dirname: &str) -> PathBuf {
    let mut save_dir = env::current_dir().unwrap();
    save_dir.push(dirname);
    if !save_dir.exists() {
        create_dir_all(save_dir.clone());
    }
    println!("{:?}", save_dir);
    save_dir
}

fn download(url: Url, book_dir: PathBuf) {
    let page_name = url.path_segments().unwrap();
    let result = file_fetcher::open_bytes(url).unwrap();
    let mut save_path = env::current_dir().unwrap();
    save_path.push("fucking_good.png");

    let mut file = File::create(save_path).unwrap();
    file.write_all(&*result);
}

fn send_request(url: Url) -> errors::ComicResult<String> {
    let mut headers = Headers::new();
    headers.set_raw("Content-Type", "text/html;charset=UTF-8");

    let client = reqwest::Client::new();
    let mut resp = client.request(Method::Get, url).headers(headers).send()?;

    println!("status {} ", resp.status().is_success());

    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();

    let body = GBK.decode(&*buf, DecoderTrap::Replace).unwrap();

    Ok(body)
}
