use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, StatusCode, Request, Response, Server, Method};
use std::{fs};
use std::env;



async fn response(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path();

    match (method, path) {
        (Method::GET, "/flips") => {
            let resp: serde_json::Value = get_flips().await;
            Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Allow-Methods", "GET")
            .body(Body::from(serde_json::to_vec(&resp).unwrap()))
            .unwrap())
        }
        
        (Method::GET, "/targets") => {
            let resp: serde_json::Value = get_targets().await;
            Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Allow-Methods", "GET")
            .body(Body::from(serde_json::to_vec(&resp).unwrap()))
            .unwrap())
        }

        (_method, _path) => { 
            Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Allow-Methods", "GET")
            .body(Body::from(serde_json::to_vec("Invalid Endpoint!").unwrap()))
            .unwrap())
        }

        // (Method::GET, "/target")
    }
    
}


pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(response)) }
    });

    let addr = ([0, 0, 0, 0], get_port()).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}



async fn get_flips() -> serde_json::Value {
    let file: String = String::from("./files/flips.json");
    let next_update = fs::read_to_string("./files/next_update.txt").expect("Unable to read file");
    let flips: String = fs::read_to_string(&file).expect("Unable to read file");
    let str_json = format!("[{{\"next_update\": {}, \"flips\": {}}}]", next_update, flips);
    let json: serde_json::Value = serde_json::from_str(&str_json).unwrap();
    // Ok(warp::reply::json(&load_json))
    json
}

async fn get_targets() -> serde_json::Value {
    let file: String = String::from("./files/targets.json");
    let targets: String = fs::read_to_string(&file).expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&targets).unwrap();
    // Ok(warp::reply::json(&load_json))
    json
}

fn get_port() -> u16 {
    env::var("PORT").unwrap_or_else(|_| 3000.to_string()).to_string().parse::<u16>().unwrap()
}