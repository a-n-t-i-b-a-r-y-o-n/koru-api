/*
        koru-api - Roku Remote APIs
*/

mod db_helper;
mod serialization;

#[allow(unused_attributes)]
#[cfg_attr(feature = "rest",)]
mod rest_server;
mod rest_handlers;

#[allow(unused_attributes)]
#[cfg_attr(feature = "mqtt",)]
mod mqtt_client;

#[tokio::main]
async fn main() {

    // Conditionally start REST API
    if true /*cfg!(rest)*/ {
        rest_server::start([127,0,0,1], 3030).await
    }

    if true /*cfg!(mqtt)*/ {
        //mqtt_client::start().await;
    }

}

