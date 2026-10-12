// SPDX-License-Identifier: MIT

//! Query test for the ring subsystem.
//!
//! The loopback interface does not support rings, netdevsim is required.
//! See [common::netdevsim_iface] for how to instantiate one, the test skips
//! when netdevsim is not available.

mod common;

use ethtool::{EthtoolAttr, EthtoolCmd, EthtoolHeader, EthtoolRingAttr};
use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_rings_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped query_rings_netdevsim: netdevsim is not available");
        return;
    };

    let mut handle = common::connect().await;
    let mut stream = handle
        .ring()
        .get(Some(&iface_name))
        .execute()
        .await
        .expect("failed to request the ring parameters");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a ring reply"));
    }

    assert_eq!(msgs.len(), 1);
    let ethtool_msg = &msgs[0].payload;
    assert_eq!(ethtool_msg.cmd, EthtoolCmd::RingGetReply);
    match &ethtool_msg.nlas[0] {
        EthtoolAttr::Ring(EthtoolRingAttr::Header(headers)) => {
            assert!(
                headers.contains(&EthtoolHeader::DevName(iface_name.clone())),
                "unexpected ring header {headers:?}"
            );
        }
        nla => panic!("unexpected first ring attribute {nla:?}"),
    }
}
