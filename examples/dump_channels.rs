// SPDX-License-Identifier: MIT

use futures_util::stream::StreamExt;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(get_channels(None));
}

async fn get_channels(iface_name: Option<&str>) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut channel_handle =
        handle.channel().get(iface_name).execute().await.unwrap();

    let mut msgs = Vec::new();
    while let Some(Ok(msg)) = channel_handle.next().await {
        msgs.push(msg);
    }
    assert!(!msgs.is_empty());
    for msg in msgs {
        println!("{msg:?}");
    }
}
