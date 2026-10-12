// SPDX-License-Identifier: MIT

//! Query test for the timestamping subsystem.

mod common;

use ethtool::{EthtoolAttr, EthtoolCmd, EthtoolHeader, EthtoolTsInfoAttr};
use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_tsinfo_lo() {
    let mut handle = common::connect().await;

    // The loopback interface provides software timestamping.
    let mut stream = handle
        .tsinfo()
        .get(Some("lo"))
        .execute()
        .await
        .expect("failed to request the timestamping information of `lo`");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a timestamping reply"));
    }

    assert_eq!(msgs.len(), 1);
    let ethtool_msg = &msgs[0].payload;
    assert_eq!(ethtool_msg.cmd, EthtoolCmd::TsInfoGetReply);
    match &ethtool_msg.nlas[0] {
        EthtoolAttr::TsInfo(EthtoolTsInfoAttr::Header(headers)) => {
            assert!(
                headers.contains(&EthtoolHeader::DevName("lo".to_string())),
                "unexpected timestamping header {headers:?}"
            );
        }
        nla => panic!("unexpected first timestamping attribute {nla:?}"),
    }
}
