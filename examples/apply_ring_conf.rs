// SPDX-License-Identifier: MIT

use std::env;

use ethtool::EthtoolTcpDataSplit;

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
    rt.block_on(apply_ring_conf(link_name));
}

async fn apply_ring_conf(iface_name: &str) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    // Fetch the current ring parameters, change what is wanted and apply
    // them back. Fields left as None are not touched by the kernel.
    let mut config = handle.get_ring(iface_name).await.unwrap();
    config.rx = config.rx_max;
    config.tcp_data_split = Some(EthtoolTcpDataSplit::Enabled);

    let result = handle.apply_ring(config).await;

    if let Err(error) = result {
        panic!("{:?}", error);
    }
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example apply_ring_conf -- <link name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd ethtool ; cargo build --example apply_ring_conf

Then find the binary in the target directory:

    cd target/debug/examples ; sudo ./apply_ring_conf <link_name>"
    );
}
