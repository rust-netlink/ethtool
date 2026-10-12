// SPDX-License-Identifier: MIT

//! Query and set tests for the pause subsystem.
//!
//! The loopback interface does not support pause, netdevsim is required.
//! See [common::netdevsim_iface] for how to instantiate one, the tests skip
//! when netdevsim is not available.

mod common;

use ethtool::EthtoolPauseParsed;

async fn get_pause(iface_name: &str) -> EthtoolPauseParsed {
    let mut handle = common::connect().await;

    handle
        .get_pause(iface_name)
        .await
        .expect("failed to request the pause configuration")
}

#[tokio::test]
async fn get_pause_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped get_pause_netdevsim: netdevsim is not available");
        return;
    };

    let config = get_pause(&iface_name).await;

    assert_eq!(config.iface_name.as_deref(), Some(iface_name.as_str()));
    assert!(config.autoneg.is_some());
}

#[tokio::test]
async fn dump_pause_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped dump_pause_netdevsim: netdevsim is not available");
        return;
    };

    let mut handle = common::connect().await;
    let configs = handle
        .dump_pause()
        .await
        .expect("failed to dump the pause configurations");

    let config = configs
        .iter()
        .find(|config| {
            config.iface_name.as_deref() == Some(iface_name.as_str())
        })
        .expect("the netdevsim interface is missing from the dump");
    assert!(config.autoneg.is_some());
}

#[tokio::test]
async fn set_pause_netdevsim() {
    let Some(iface_name) = common::netdevsim_iface() else {
        eprintln!("skipped set_pause_netdevsim: netdevsim is not available");
        return;
    };

    let mut config = EthtoolPauseParsed::default();
    config.iface_name = Some(iface_name.clone());
    config.autoneg = Some(false);
    config.rx = Some(true);
    config.tx = Some(true);

    let mut handle = common::connect().await;
    if let Err(err) = handle.apply_pause(config).await {
        if common::is_skippable(&err) {
            eprintln!("skipped set_pause_netdevsim: {err}");
            return;
        }
        panic!("failed to apply the pause configuration: {err}");
    }

    let config = get_pause(&iface_name).await;

    assert_eq!(config.autoneg, Some(false));
    assert_eq!(config.rx, Some(true));
    assert_eq!(config.tx, Some(true));
}
