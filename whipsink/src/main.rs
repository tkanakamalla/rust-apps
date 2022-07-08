use std::env;
use std::error::Error;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use parse_link_header;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let url = &args[1];
    println!("url is {}", url);
    let client = Client::new();
    let resp = client.request(reqwest::Method::OPTIONS,url).send()?;
//    let resp = reqwest::blocking::post(url)?.text()?;
    match resp.status() {
        StatusCode::OK => println!("success"),
        StatusCode::NO_CONTENT => {
            let headermap = resp.headers();
            for (key, value) in headermap.iter() {
                println!("{:?}: {:?}", key, value);
            }
            let links = headermap.get_all("link");
            for link in links.iter() {
            //    println!("{:?}", link);
                               
                let item_map = parse_link_header::parse_with_rel(link.to_str().expect("String conv failed"));
                //assert!(item_map.is_ok());
                match item_map {
                        Ok(res) => println! ("Result {:?} {}",res, link.to_str().unwrap()),
                        Err(e) => println! ("Error {:?} {}", e, link.to_str().unwrap()),
                }
                // for item in item_map.iter() {
                //     println! {".."};
                //     println!("{:?}", item);
                // }
            }
            // let mut iter = links.iter();
            // loop {
            //     // if iter.next().is_none() {
            //     //     break;
            //     // }
            //     println!("{:?}",iter.next().unwrap());
                
            // }
        },
        s => println!("received: {}", s),
    };
Ok(())
}
