use select::document::Document;
use select::predicate::Attr;
use reqwest::blocking::get;
use std::fs::{write,read_to_string};
fn main() {
    println!("link target (http://sample.x .mkv)");
    // get link and taget from user { http://sample.x:.mkv }
    let link_target:Vec<String> = {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).unwrap();
        temp.split(" ").map(|s| {s.to_string()}).collect()
    };
    println!("{:?}",link_target);
    let raw_html = get(&link_target[0]).unwrap().text().unwrap();
    //write raw_html to file
    write("./raw.html", &raw_html).unwrap();
    
    //part 2
    // read html
    let html = read_to_string("./raw.html").expect("**[can't read file]**");
    // make document scraper
    let doc = Document::from(html.as_str());
    // string temp to keep links
    let mut temp = String::new();
    // get nodes
    for node in doc.find(Attr("href",())){
        let t = node.attr("href").unwrap();
        if t.contains(format!("{}",link_target[1].replace("\r\n", "")).as_str()){
            if t.contains("http") ||t.contains("www."){
                let tt = format!("{}\n",t);
                temp.push_str(&tt);
            }
            else{
                let tt = format!("{}{}\n",link_target[0],t);
                temp.push_str(&tt);
            }
        }
    };
    write("./links.txt", temp).unwrap();
}