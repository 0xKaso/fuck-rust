use std::process::Output;

use std::fs;
fn main() {
    let url = "https://move-book.com/cn/introduction/getting-started.html";
    let output = "rust.md";

    println!("内容来自于：{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("转换成MD中...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("转换成功,{}", output);
}
