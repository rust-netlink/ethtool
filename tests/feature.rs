// SPDX-License-Identifier: MIT

//! Query and set tests for the feature subsystem.
//!
//! The set tests need netdevsim, whose `hw-tc-offload` feature can be
//! changed. See [common::netdevsim_iface] for how to instantiate one, the
//! tests skip when netdevsim is not available.

mod common;

use ethtool::{
    EthtoolAttr, EthtoolBitsetValue, EthtoolCmd, EthtoolError,
    EthtoolFeatureAttr, EthtoolHeader,
};
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

/// Whether `feature` is active on `iface_name`.
async fn is_active(iface_name: &str, feature: &str) -> bool {
    let mut handle = common::connect().await;
    let mut stream = handle
        .feature()
        .get(Some(iface_name))
        .execute()
        .await
        .expect("failed to request the features");
    let mut active = None;
    while let Some(reply) = stream.next().await {
        for nla in reply.expect("failed to parse a feature reply").payload.nlas
        {
            if let EthtoolAttr::Feature(EthtoolFeatureAttr::Active(bitset)) =
                nla
            {
                let EthtoolBitsetValue::Bits(bits) = bitset.value else {
                    panic!("unexpected compact active bitset");
                };
                active = Some(
                    bits.iter().any(|bit| bit.name == feature && bit.value),
                );
            }
        }
    }
    active.expect("the reply has no active features")
}

#[tokio::test]
async fn set_features_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped set_features_netdevsim: netdevsim is not available");
        return;
    };

    for enable in [true, false] {
        let mut handle = common::connect().await;
        let unapplied = match handle
            .feature()
            .set(&iface_name)
            .feature("hw-tc-offload", enable)
            .execute()
            .await
        {
            Ok(unapplied) => unapplied,
            Err(err) if common::is_skippable(&err) => {
                eprintln!("skipped set_features_netdevsim: {err}");
                return;
            }
            Err(err) => panic!("failed to set the features: {err}"),
        };
        assert!(unapplied.is_empty(), "{unapplied:?}");
        assert_eq!(is_active(&iface_name, "hw-tc-offload").await, enable);
    }
}

#[tokio::test]
async fn set_unchangeable_feature_lo() {
    let mut handle = common::connect().await;

    // `loopback` is a fixed feature of `lo`, the request fails before
    // anything is sent.
    let err = handle
        .feature()
        .set("lo")
        .feature("loopback", false)
        .execute()
        .await
        .unwrap_err();

    assert!(matches!(err, EthtoolError::InvalidInput(_)), "{err:?}");
}
