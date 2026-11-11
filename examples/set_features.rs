// SPDX-License-Identifier: MIT

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || !["on", "off"].contains(&args[3].as_str()) {
        usage();
        return;
    }
    let link_name = &args[1];
    let feature = &args[2];
    let enable = args[3] == "on";
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(set_features(link_name, feature, enable));
}

async fn set_features(iface_name: &str, feature: &str, enable: bool) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let unapplied = handle
        .feature()
        .set(iface_name)
        .feature(feature, enable)
        .execute()
        .await;

    match unapplied {
        Ok(unapplied) if unapplied.is_empty() => {}
        Ok(unapplied) => panic!("the driver refused {unapplied:?}"),
        Err(error) => panic!("{:?}", error),
    }
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example set_features -- <link name> <feature> on|off

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cd ethtool ; cargo build --example set_features

Then find the binary in the target directory:

    cd target/debug/examples ; sudo ./set_features <link_name> <feature> on|off"
    );
}
