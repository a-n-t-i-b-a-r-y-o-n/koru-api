use sqlx::sqlite::SqliteRow;
use koru::Device;
use sqlx::Row;
use std::collections::HashMap;

/// Helper methods for serialization and deserialization

/// Trait and impl to deserialize to koru::Device
pub trait _AsKoruDevice {
    fn to_koru_device(&self) -> koru::Device;
}
// Deserialize Device from DB record
impl _AsKoruDevice for SqliteRow {
    fn to_koru_device(&self) -> koru::Device {
        Device {
            ipv4: self.try_get(1).unwrap(),
            port: self.try_get(2).unwrap(),
            name: self.try_get(3).unwrap(),
            network: {
                match self.try_get(4).unwrap() {
                    "ETHERNET" => koru::NETWORKTYPE::ETHERNET,
                    "WIRELESS" => koru::NETWORKTYPE::WIRELESS,
                    _ => koru::NETWORKTYPE::WIRELESS
                }
            },
            mac_wlan: split_mac(self.try_get(5).unwrap()),
            mac_eth: split_mac(self.try_get(6).unwrap()),
        }
    }
}

/// Helper to split up device MACs into byte arrays
fn split_mac(input: &str) -> [u8; 6] {
    let mut index = 0;
    let mut output: [u8; 6] = [0; 6];
    str::split(input, ':')
        .for_each(|chunk| {
            output[index] = u8::from_str_radix(chunk, 16).unwrap();
            index += 1;
        });
    output
}

/// Trait and impl to serialize koru::Device into HashMap<String, String> (for use with json!() )
pub trait _AsHashMap {
    fn as_hash_map(&self) -> HashMap<String, String>;
}
impl _AsHashMap for Device {
    fn as_hash_map(&self) -> HashMap<String, String> {
        let mut output: HashMap<String, String> = HashMap::new();
        output.insert("ipv4".to_string(), String::from(&self.ipv4));
        output.insert("port".to_string(), format!("{}", &self.port));
        output.insert("name".to_string(), String::from(&self.name));
        output.insert("network".to_string(), String::from(&self.network.to_string()));
        output.insert("mac_wlan".to_string(), format!("{:?}", &self.mac_wlan));
        output.insert("mac_eth".to_string(), format!("{:?}", &self.mac_eth));
        output
    }
}