// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use ethtool::{
    EthtoolCableTestTdrConfig, EthtoolCmd, EthtoolError, EthtoolMessage,
};
use futures::{StreamExt, TryStreamExt};
use netlink_packet_core::{
    NetlinkMessage, NetlinkPayload, ParseableParametrized, NLM_F_REQUEST,
};
use netlink_packet_generic::{
    ctrl::{
        nlas::{GenlCtrlAttrs, McastGrpAttrs},
        GenlCtrl, GenlCtrlCmd,
    },
    GenlFamily, GenlMessage,
};
use netlink_sys::AsyncSocket;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return Ok(());
    }
    let link_name = &args[1];
    cable_test_tdr(link_name).await
}

async fn cable_test_tdr(iface_name: &str) -> Result<()> {
    // Obtain the multicast group ID for "monitor".
    let multicast_id = get_multicast_id().await?;
    println!("Found Monitor Multicast Group with ID: {multicast_id}");

    // Set up a new ethtool netlink connection and subscribe to the multicast
    // group.
    let (mut connection, mut handle, mut messages) = ethtool::new_connection()?;
    {
        let socket = connection.socket_mut().socket_mut();
        socket.bind_auto()?;
        socket.add_membership(multicast_id)?;
    }

    // Spawn the connection.
    tokio::spawn(connection);

    // Start the cable test.
    let iface = iface_name.to_string();
    tokio::spawn(async move {
        let config = EthtoolCableTestTdrConfig {
            first: Some(500),
            last: None,
            step: Some(200),
            pair: None,
        };

        let mut stream = handle
            .cable_test_tdr()
            .start_with_config(&iface, config)
            .execute()
            .await;
        match stream.try_next().await {
            Ok(_) => println!(
                "Started Cable Test TDR. This might take a few seconds."
            ),
            Err(e) => {
                print_ethtool_error(e);
                std::process::exit(1);
            }
        }
    });

    // Process incoming netlink messages, filtering for cable test
    // notifications.
    while let Some((msg, _)) = messages.next().await {
        if let NetlinkPayload::InnerMessage(inner) = &msg.payload {
            let ethtool_msg =
                EthtoolMessage::parse_with_param(&inner.payload, inner.header)?;
            if ethtool_msg.cmd == EthtoolCmd::CableTestTdrNotify {
                dbg!(ethtool_msg);
            }
        }
    }

    Ok(())
}

fn print_ethtool_error(error: EthtoolError) {
    match error {
        EthtoolError::NetlinkError(message) => {
            if let Some(code) = message.code {
                let os_error = std::io::Error::from_raw_os_error(-code.get());
                eprintln!(
                    "Netlink error occurred: OS error code: {}",
                    os_error
                );
            } else {
                eprintln!("Netlink error occurred: Unknown error code");
            }
        }
        _ => eprintln!("Other error occurred: {error:?}"),
    }
}

async fn get_multicast_id() -> Result<u32> {
    let (connection, mut ethtool, _) = ethtool::new_connection()?;
    let _ = tokio::spawn(connection);

    // Build netlink control message.
    let mut msg: NetlinkMessage<_> = GenlMessage::from_payload(GenlCtrl {
        cmd: GenlCtrlCmd::GetFamily,
        nlas: vec![GenlCtrlAttrs::FamilyName(
            EthtoolMessage::family_name().to_owned(),
        )],
    })
    .into();

    msg.header.message_type =
        ethtool.handle.resolve_family_id::<EthtoolMessage>().await?;
    msg.header.flags = NLM_F_REQUEST;

    // Receive response from control message request.
    let responses = ethtool.handle.request(msg).await?;

    let monitor_id = responses.try_filter_map(async move |response| {
        // Only care about generic messages with inner messages.
        let NetlinkPayload::InnerMessage(gen) = &response.payload else {
            return Ok(None);
        };

        // Check for ethtool family in the nals.
        let has_ethtool_family = gen.payload.nlas.iter().any(|nla| {
            matches!(nla, GenlCtrlAttrs::FamilyName(name) if name == EthtoolMessage::family_name())
        });

        // Only care about NewFamily announcements.
        if !has_ethtool_family || gen.payload.cmd != GenlCtrlCmd::NewFamily {
            return Ok(None);
        }

        // Extract id on groups where name is "monitor".
        let id = gen.payload.nlas.iter().find_map(|nla| {
            let GenlCtrlAttrs::McastGroups(groups) = nla else {
                return None;
            };

            groups.iter().find_map(|group| {
                let mut id: Option<u32> = None;
                let mut name: Option<&str> = None;

                for a in group {
                    match a {
                        McastGrpAttrs::Id(v) => id = Some(*v),
                        McastGrpAttrs::Name(n) => name = Some(n.as_str()),
                    }
                }

                (name == Some("monitor")).then_some(id?)
            })
        });

        Ok(id)
    })
    .try_collect::<Vec<_>>()
    .await?
    .into_iter()
    .next()
    .context("ethtool multicast monitor id not found")?;

    Ok(monitor_id)
}

fn usage() {
    eprintln!(
        "Usage:
    cargo run --example cable_test_tdr -- <link_name>

Note: This program requires root privileges. It is recommended to build the example first:

    cd ethtool
    cargo build --example cable_test_tdr

Then run the binary with sudo:

    cd target/debug/examples
    sudo ./cable_test_tdr <link_name>"
    );
}
