// SPDX-License-Identifier: MIT

//! Query test for the feature subsystem.

mod common;

use ethtool::{EthtoolAttr, EthtoolCmd, EthtoolFeatureAttr, EthtoolHeader};
use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_features_lo() {
    let mut handle = common::connect().await;

    let mut stream = handle
        .feature()
        .get(Some("lo"))
        .execute()
        .await
        .expect("failed to request the features of `lo`");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a feature reply"));
    }

    assert_eq!(msgs.len(), 1);
    let ethtool_msg = &msgs[0].payload;

    assert_eq!(ethtool_msg.cmd, EthtoolCmd::FeatureGetReply);
    assert!(ethtool_msg.nlas.len() > 1);
    assert_eq!(
        ethtool_msg.nlas[0],
        EthtoolAttr::Feature(EthtoolFeatureAttr::Header(vec![
            EthtoolHeader::DevIndex(1),
            EthtoolHeader::DevName("lo".into())
        ]))
    );
}
