// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    let iface_name = std::env::args().nth(1);
    rt.block_on(get_eeprom(iface_name.as_deref()));
}

async fn get_eeprom(iface_name: Option<&str>) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut eeprom_handle = handle
        .eeprom()
        .get(iface_name, 0, 1, 0, 0, 0x50)
        .execute()
        .await
        .unwrap();

    let mut msgs = Vec::new();
    while let Some(Ok(msg)) = eeprom_handle.next().await {
        msgs.push(msg);
    }
    assert!(!msgs.is_empty());
    for msg in msgs {
        println!("{:?}", msg);
    }
}
