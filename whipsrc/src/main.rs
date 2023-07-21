
use std::net::SocketAddr;
use warp::{http::{HeaderMap, HeaderValue, self}, Filter, Reply, hyper::{header::{LINK, CONTENT_TYPE}, Body}, reply::Response};
use serde::Deserialize;
use serde::Serialize;

const ENDPOINT: &str = "whip";
const IP_ADDR: &str = "127.0.0.1:8080";
const STUN_SERVER: Option<&str> = Some("stun://stun.l.google.com:19302");
const TURN_SERVERS: Option<&str> = Some("stun://turn.l.google.com:19302");


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

async fn endp1_handler(
    item: Item,
    ) -> Result<impl warp::Reply, warp::Rejection> {

    let body = warp::reply::json(&item);
    let reply = warp::reply::with_status(body, http::StatusCode::OK);

    let mut links = HeaderMap::new();
    if STUN_SERVER.is_some() {
        links.append("link", HeaderValue::from_static(STUN_SERVER.unwrap()));
    }
    if TURN_SERVERS.is_some() {
    links.append("link", HeaderValue::from_static(TURN_SERVERS.unwrap()));
    }

    let mut res = reply.into_response();
    let headers = res.headers_mut();
    headers.extend(links);

    Ok(res)
}

async fn endp_handler(
    content_type: String,
    body: warp::hyper::body::Bytes,
    ) -> Result<impl warp::Reply, warp::Rejection> {

    let reply = warp::reply::reply();

    if content_type != "application/sdp" {
        let res = warp::reply::with_status(reply, http::StatusCode::UNSUPPORTED_MEDIA_TYPE);
        return Ok(res.into_response())
    }

    let mut links = HeaderMap::new();
    if STUN_SERVER.is_some() {
        links.append(LINK, HeaderValue::from_static(STUN_SERVER.unwrap()));
    }
    if TURN_SERVERS.is_some() {
        links.append(LINK, HeaderValue::from_static(TURN_SERVERS.unwrap()));
    }


    let mut res =
         http::Response::builder()
        .header(CONTENT_TYPE, "application/sdp")
        .body(Body::from("hello"))
        .unwrap();

    let headers = res.headers_mut();
    headers.extend(links);

    Ok(res)
}

// fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     // (and to reject huge payloads)...
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

#[tokio::main]
async fn main() {

    let addr: SocketAddr = IP_ADDR.parse().expect("Unable to parse");

    // POST /endpoint
    let endp_filter = warp::post()
        .and(warp::path(ENDPOINT))
        .and(warp::path::end())
        .and(warp::header("content-type"))
        .and(warp::body::bytes())
        .and_then(endp_handler);

    warp::serve(endp_filter)
        .run(addr)
        .await;
}