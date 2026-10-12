// SPDX-License-Identifier: MIT

use std::env;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    env_logger::init();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(run());
}

async fn run() {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    match env::args().nth(1) {
        Some(iface_name) => {
            let config = handle.get_pause(&iface_name).await.unwrap();
            println!("{config:?}");
        }
        None => {
            for config in handle.dump_pause().await.unwrap() {
                println!("{config:?}");
            }
        }
    }
}
