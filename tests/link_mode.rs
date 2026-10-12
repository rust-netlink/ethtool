// SPDX-License-Identifier: MIT

//! Query test for the link mode subsystem.

mod common;

use ethtool::EthtoolCmd;
use futures_util::stream::StreamExt;

#[tokio::test]
// CI container normally have a veth for external communication which support
// link modes of ethtool.
async fn query_link_modes_dump() {
    let mut handle = common::connect().await;

    let mut stream = handle
        .link_mode()
        .get(None)
        .execute()
        .await
        .expect("failed to request the link modes");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a link mode reply"));
    }

    assert!(!msgs.is_empty());
    let ethtool_msg = &msgs[0].payload;
    println!("ethtool_msg {ethtool_msg:?}");

    assert_eq!(ethtool_msg.cmd, EthtoolCmd::LinkModeGetReply);
    assert!(ethtool_msg.nlas.len() > 1);
}
