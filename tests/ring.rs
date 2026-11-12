// SPDX-License-Identifier: MIT

//! Query and set tests for the ring subsystem.
//!
//! The loopback interface does not support rings, netdevsim is required.
//! See [common::netdevsim_iface] for how to instantiate one, the test skips
//! when netdevsim is not available.

mod common;

use ethtool::{
    EthtoolAttr, EthtoolCmd, EthtoolHeader, EthtoolRingAttr, EthtoolRingParsed,
    EthtoolTcpDataSplit,
};
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

#[tokio::test]
async fn set_ring_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped set_ring_netdevsim: netdevsim is not available");
        return;
    };

    let mut handle = common::connect().await;
    let current = handle
        .get_ring(&iface_name)
        .await
        .expect("failed to request the ring parameters");
    assert_eq!(current.iface_name.as_deref(), Some(iface_name.as_str()));

    let mut config = EthtoolRingParsed::default();
    config.iface_name = Some(iface_name.clone());
    config.rx = Some(256);
    config.tcp_data_split = Some(EthtoolTcpDataSplit::Enabled);
    if let Err(err) = handle.apply_ring(config).await {
        if common::is_skippable(&err) {
            eprintln!("skipped set_ring_netdevsim: {err}");
            return;
        }
        panic!("failed to apply the ring parameters: {err}");
    }

    let config = handle
        .get_ring(&iface_name)
        .await
        .expect("failed to request the ring parameters");
    assert_eq!(config.rx, Some(256));
    assert_eq!(config.tcp_data_split, Some(EthtoolTcpDataSplit::Enabled));
}
