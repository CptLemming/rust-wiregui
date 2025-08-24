use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use gpui::SharedString;
use networkmanager::NetworkManager;
use networkmanager::devices::{Any, Device};

use dbus::blocking::Connection;

use crate::connections::ConnectionItem;

const CONNECTION_ACTIVATED: u32 = 100;

pub fn get_connection_list() -> Vec<ConnectionItem> {
    let dbus_connection = Connection::new_system().expect("Failed to open D-BUS");

    let nm = NetworkManager::new(&dbus_connection);

    let mut out = vec![];

    for device in nm.get_all_devices().expect("Failed to fetch devices") {
        match device {
            Device::Wireguard(wireguard) => {
                // Found a wireguard connection, look up it's name and state
                match (wireguard.interface(), wireguard.state()) {
                    (Ok(name), Ok(state)) => {
                        out.push(ConnectionItem {
                            name: name.into(),
                            active: state == CONNECTION_ACTIVATED,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    out
}

pub fn activate_connection(name: SharedString) -> Result<(), io::Error> {
    Command::new("nmcli")
        .args(["connection", "up", &name.to_string()])
        .output()
        .inspect(|res| {
            println!("Activate : {res:?}");
        })
        .inspect_err(|e| {
            eprintln!("Activate : {e}");
        })?;

    Ok(())
}

pub fn deactivate_connection(name: SharedString) -> Result<(), io::Error> {
    Command::new("nmcli")
        .args(["connection", "down", &name.to_string()])
        .output()
        .inspect(|res| {
            println!("Deactivate : {res:?}");
        })
        .inspect_err(|e| {
            eprintln!("Deactivate : {e}");
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::net::get_connection_list;

    #[test]
    pub fn test_connections() {
        get_connection_list();
    }
}
