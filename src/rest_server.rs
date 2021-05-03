use warp::Filter;
use serde_json::json;
use std::net::SocketAddrV4;
use std::str::FromStr;

use crate::rest_handlers::*;

#[allow(dead_code)]
pub async fn start(ipv4: [u8; 4], port: i32) {

    /* --- Debug Endpoints --- */
    // GET /version - Returns version number of koru-api
    let version = warp::path!("version")
        .map(|| json!(env!("CARGO_PKG_VERSION")).to_string());

    // GET /devices - Returns a JSON array of devices in the database
    let dump_devices = warp::path!("devices")
        .and_then(|| async move { dump_db_devices().await });

    /* --- Device discovery Endpoints --- */

    // GET /discover - Returns a JSON array of devices discovered on the network w/ socket timeout of 5 seconds
    let discover = warp::path!("discover")
        .and_then(|| async move { discover_devices(5).await });

    // GET /discover/$seconds - Identical to GET /discover, only with a configurable timeout
    let discover_wait = warp::path!("discover" / u64)
        .and_then(|seconds: u64| async move { discover_devices(seconds).await });

    /* --- Device Endpoints --- */

    // GET /device/$id/info - JSON-formatted version of device info XML
    let device_info = warp::path!("device" / i32 / "info")
        .and_then(|id: i32| async move { get_device_info(id).await });

    // GET /device/$id/power - Returns current power state
    let power_state = warp::path!("device" / i32 / "power")
        .and_then(|id: i32| async move { get_power_state(id).await });

    // GET /device/$id/power/$command - Sends a power command (e.g. toggle, turnoff)
    let power_command = warp::path!("device" / i32 / "power" / String)
        .and_then(|id: i32, command: String| async move { send_power_command(id, command).await });

    // GET /device/$id/launch/$app - Launch an app by id
    let launch_app = warp::path!("device" / i32 / "launch" / i32)
        .and_then(|id: i32, appid: i32| async move { launch_app_id(id, appid).await });

    // GET /device/$id/button/$button - Emulates remote button press
    let press_button = warp::path!("device" / i32 / "button" / String)
        .and_then(|id: i32, button: String| async move { send_button(id, button).await });

    // GET /device/$id/key/$char - Sends UTF-8 character press
    let press_key = warp::path!("device" / i32 / "key" / char)
        .and_then(|id: i32, key: char| async move { send_key(id, key).await });

    // GET /device/$id/string/$string - Sends UTF-8 string as series of key presses
    let send_string = warp::path!("device" / i32 / "string" / String)
        .and_then(|id: i32, s: String| async move { send_string(id, s).await });

    let routes = warp::get()
        .and(version
            .or(dump_devices)
            .or(discover)
            .or(discover_wait)
            .or(device_info)
            .or(power_state)
            .or(power_command)
            .or(launch_app)
            .or(press_button)
            .or(press_key)
            .or(send_string)
        );
    warp::serve(routes)
        .run(SocketAddrV4::from_str(format!("{}.{}.{}.{}:{}", ipv4[0], ipv4[1], ipv4[2], ipv4[3], port).as_str()).unwrap())
        .await
}