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
    rt.block_on(apply_pause_conf(link_name));
}

async fn apply_pause_conf(iface_name: &str) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    // Fetch the current configuration, change what is wanted and apply it
    // back. Fields left as None are not touched by the kernel.
    let mut config = handle.get_pause(iface_name).await.unwrap();
    config.rx = Some(true);
    config.tx = Some(true);

    let result = handle.apply_pause(config).await;

    if let Err(error) = result {
        panic!("{:?}", error);
    }
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example apply_pause_conf -- <link name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd ethtool ; cargo build --example apply_pause_conf

Then find the binary in the target directory:

    cd target/debug/examples ; sudo ./apply_pause_conf <link_name>"
    );
}
