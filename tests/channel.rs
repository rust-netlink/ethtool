// SPDX-License-Identifier: MIT

//! Query and set tests for the channel subsystem.
//!
//! The loopback interface does not support channels, netdevsim is required.
//! See [common::netdevsim_iface] for how to instantiate one, the tests skip
//! when netdevsim is not available.

mod common;

use ethtool::{EthtoolAttr, EthtoolChannelAttr, EthtoolCmd, EthtoolHeader};
use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_channels_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!(
            "skipped query_channels_netdevsim: netdevsim is not available"
        );
        return;
    };

    let mut handle = common::connect().await;
    let mut stream = handle
        .channel()
        .get(Some(&iface_name))
        .execute()
        .await
        .expect("failed to request the channels");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        msgs.push(reply.expect("failed to parse a channel reply"));
    }

    assert_eq!(msgs.len(), 1);
    let ethtool_msg = &msgs[0].payload;
    assert_eq!(ethtool_msg.cmd, EthtoolCmd::ChannelGetReply);
    match &ethtool_msg.nlas[0] {
        EthtoolAttr::Channel(EthtoolChannelAttr::Header(headers)) => {
            assert!(
                headers.contains(&EthtoolHeader::DevName(iface_name.clone())),
                "unexpected channel header {headers:?}"
            );
        }
        nla => panic!("unexpected first channel attribute {nla:?}"),
    }
}

#[tokio::test]
async fn set_channel_count_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!(
            "skipped set_channel_count_netdevsim: netdevsim is not available"
        );
        return;
    };

    let mut handle = common::connect().await;
    let result = handle
        .channel()
        .set(&iface_name)
        .combined_count(1)
        .execute()
        .await;

    if let Err(err) = result {
        if common::is_skippable(&err) {
            eprintln!("skipped set_channel_count_netdevsim: {err}");
            return;
        }
        panic!("failed to set the channel count: {err}");
    }
}
