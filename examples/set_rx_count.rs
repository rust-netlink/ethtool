// SPDX-License-Identifier: MIT

use std::env;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return;
    }
    let link_name = &args[1];
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(set_rx_count(link_name));
}

async fn set_rx_count(iface_name: &str) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let result = handle.channel().set(iface_name).rx_count(4).execute().await;

    if let Err(error) = result {
        panic!("{:?}", error);
    }
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example set_rx_count -- <link name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd ethtool ; cargo build --example set_rx_count

Then find the binary in the target directory:

    cd target/debug/example ; sudo ./set_rx_count <link_name>"
    );
}
