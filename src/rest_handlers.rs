use crate::db_helper;
use crate::serialization::*;
use std::convert::Infallible;
use serde_json::json;
use koru::POWERCOMMAND;
use std::time::Duration;

pub async fn get_device_info(id: i32) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        Ok(json!(device.get_info().await.unwrap()).to_string())
    } else {
        Ok(json!({"Error" : "Unknown Device"}).to_string())
    }
}

pub async fn get_power_state(id: i32) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        let state = device.get_power_state().await;
        Ok(json!({"PowerState" : state.to_string()}).to_string())
    } else {
        Ok(json!({"Error" : "Unknown Device"}).to_string())
    }
}

pub async fn send_power_command(id: i32, command: String) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        let power_command = match command.to_ascii_uppercase().as_str() {
            "OFF" | "TURNOFF" => POWERCOMMAND::TURNOFF,
            "TOGGLE" => POWERCOMMAND::TOGGLE,
            _ => POWERCOMMAND::TURNON
        };
        match device.send_power_command(power_command).await {
            Ok(_) => Ok(json!({"PowerState" : command.clone()}).to_string()),
            Err(_) => Ok(json!({"PowerState" : "Unknown"}).to_string())
        }
    } else {
        Ok(json!({"Error" : "Unknown Device"}).to_string())
    }
}

pub async fn launch_app_id(id: i32, appid: i32) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        match device.launch_app_by_id(appid).await {
            Ok(_) => Ok(json!({"Launched": appid}).to_string()),
            Err(_) => Ok(json!({"Error": "Failed to launch"}).to_string())
        }
    } else {
        Ok(json!({"Error": "Unknown Device"}).to_string())
    }
}

pub async fn send_button(id: i32, button: String) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        let _ = device.press_button(koru::BUTTON::from(button)).await;
    }
    Ok(String::new())
}

pub async fn send_key(id: i32, key: char) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        let _ = device.press_key(key).await;
    }
    Ok(String::new())
}

pub async fn send_string(id: i32, s: String) -> Result<String, Infallible> {
    if let Some(device) = db_helper::get_device(id).await {
        let _ = device.press_keys(&s).await;
    }
    Ok(String::new())
}

pub async fn discover_devices(timeout: u64) -> Result<String, Infallible> {
    if let Ok(devices) = koru::discover_devices(Duration::new(timeout, 0)).await {
        // Turn the list of device into a list of hash maps
        let mut hash_maps = Vec::new();
        devices.into_iter().for_each(|d| hash_maps.push(d.as_hash_map()));
        Ok(json!(hash_maps).to_string())
    } else {
        // Return empty dictionary
        Ok(String::from("[]"))
    }
}

pub async fn dump_db_devices() -> Result<String, Infallible> {
    if let Some(devices) = db_helper::get_all_devices().await {
        // Turn the list of device into a list of hash maps
        let mut hash_maps = Vec::new();
        devices.into_iter().for_each(|d| hash_maps.push(d.as_hash_map()));
        Ok(json!(hash_maps).to_string())
    } else {
        Ok("[]".to_string())
    }
}