use std::env;
use std::error::Error;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use parse_link_header;

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
            let links = headermap.get_all("link");
            for link in links.iter() {
            //    println!("{:?}", link);
                let link = link.to_str().expect("String conv failed");
                /*
                Note: Server appends an extra ; at the end of each link 
                but not needed as per https://datatracker.ietf.org/doc/html/rfc8288#section-3.5
                */
                let link = link.trim_matches(';');
                let item_map = parse_link_header::parse_with_rel(link);
                match item_map {
                        Ok(res) => {
                            for (key,value) in res.iter() {
                                //println!("{:?}---- {:?}\n", key, value);
                                if key == "ice-server" {
                                    println!("{}",value.raw_uri);
                                    let params = &value.params;
                                    for (k,v) in params {
                                        println! ("{}:{}",k,v);
                                    }
                                    println!("");
                                }
                            }           
                        },
                        Err(e) => println! ("Error {:?} {}", e, link),
                }
            }
        },
        s => println!("received: {}", s),
    };
Ok(())
}
