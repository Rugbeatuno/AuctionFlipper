use AuctionFlipperNode::flipper;
mod server;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _web = tokio::spawn(async {
    //     let get_items = warp::get()
    //     .and(warp::path("flips"))
    //     .and(warp::path("get"))
    //     .and(warp::path::end())
    //     .and_then(get_flips);
    
    //     let routes = get_items;
    //     warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    // });
    let _ = tokio::spawn(async {
        let _ = server::start_server().await;
    });

    flipper().await;
    Ok(())
}

