// SPDX-License-Identifier: MIT

use futures::stream::TryStreamExt;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    let iface_name = std::env::args().nth(1);
    rt.block_on(get_tsinfo(iface_name.as_deref()));
}

async fn get_tsinfo(iface_name: Option<&str>) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut tsinfo_handle = handle.tsinfo().get(iface_name).execute().await;

    let mut msgs = Vec::new();
    while let Some(msg) = tsinfo_handle.try_next().await.unwrap() {
        msgs.push(msg);
    }
    assert!(!msgs.is_empty());
    for msg in msgs {
        println!("{:?}", msg);
    }
}
