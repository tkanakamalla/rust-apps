use std::env;
use std::error::Error;
use reqwest::Client;
use reqwest::StatusCode;
use parse_link_header;
use tokio::runtime;
use futures::future;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let url = &args[1];
    println!("url is {}", url);
    let client = Client::new();
    let rt = runtime::Runtime::new().unwrap();
    let (abort_handle, abort_registration) = future::AbortHandle::new_pair();
    let canceller: future::AbortHandle = abort_handle;


    let future = client.request(reqwest::Method::OPTIONS,url).send();
    let future = async {
        match future::Abortable::new(future, abort_registration).await {
            Ok(res) => res.map_err(Some),
            Err(_) => Err(None),
        }
    };
    ctrlc::set_handler(move || {
        canceller.abort();
        println!("get ctrl c signal");
    }).expect("Error setting Ctrl-C handler");

    match rt.block_on(future) {
        Ok(r) => {},
        Err(e) => {
            println!("Failed to send request {:?}",e);
        }
    };

    // match resp.status() {
    //     StatusCode::OK => println!("success"),
    //     StatusCode::NO_CONTENT => {
    //         let headermap = resp.headers();
    //         let links = headermap.get_all("link");
    //         for link in links.iter() {
    //             println!("{:?}", link);
    //             let link = link.to_str().expect("String conv failed");
    //             /*
    //             Note: Server appends an extra ; at the end of each link 
    //             but not needed as per https://datatracker.ietf.org/doc/html/rfc8288#section-3.5
    //             */
    //             let link = link.trim_matches(';');
    //             let item_map = parse_link_header::parse_with_rel(link);
    //             match item_map {
    //                     Ok(res) => {
    //                         for (key,value) in res.iter() {
    //                             println!("{:?}---- {:?}\n", key, value);
    //                             if key == "ice-server" {
    //                                 println!("{}",value.raw_uri);
    //                                 let params = &value.params;
    //                                 for (k,v) in params {
    //                                     println! ("{}:{}",k,v);
    //                                 }
    //                                 println!("");
    //                             }
    //                         }           
    //                     },
    //                     Err(e) => println! ("Error {:?} {}", e, link),
    //             }
    //         }
    //     },
    //     s => println!("received: {}", s),
    // };
Ok(())
}
