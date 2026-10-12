// SPDX-License-Identifier: MIT

//! Query test for the FEC subsystem.
//!
//! The loopback interface does not support FEC, netdevsim is required. See
//! [common::netdevsim_iface] for how to instantiate one, the test skips
//! when netdevsim is not available.

mod common;

use ethtool::{EthtoolAttr, EthtoolCmd, EthtoolFecAttr, EthtoolHeader};
use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_fec_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped query_fec_netdevsim: netdevsim is not available");
        return;
    };

    let mut handle = common::connect().await;
    let mut stream = handle
        .fec()
        .get(Some(&iface_name))
        .execute()
        .await
        .expect("failed to request the FEC parameters");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a FEC reply"));
    }

    assert_eq!(msgs.len(), 1);
    let ethtool_msg = &msgs[0].payload;
    assert_eq!(ethtool_msg.cmd, EthtoolCmd::FecGetReply);
    match &ethtool_msg.nlas[0] {
        EthtoolAttr::Fec(EthtoolFecAttr::Header(headers)) => {
            assert!(
                headers.contains(&EthtoolHeader::DevName(iface_name.clone())),
                "unexpected FEC header {headers:?}"
            );
        }
        nla => panic!("unexpected first FEC attribute {nla:?}"),
    }
}
