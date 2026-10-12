// SPDX-License-Identifier: MIT

//! Query test for the module EEPROM subsystem.
//!
//! Module EEPROM requires SFP hardware which none of the virtual NICs
//! provide, therefore the test skips unless the queried interface has one.

mod common;

use futures_util::stream::StreamExt;

#[tokio::test]
async fn query_eeprom_lo() {
    let mut handle = common::connect().await;

    let mut stream = handle
        .eeprom()
        .get(Some("lo"), 0, 1, 0, 0, 0x50)
        .execute()
        .await
        .expect("failed to request the module EEPROM page");

    let mut msgs = Vec::new();
    while let Some(reply) = stream.next().await {
        match reply {
            Ok(msg) => msgs.push(msg),
            Err(err) if common::is_skippable(&err) => {
                eprintln!("skipped query_eeprom_lo: no module EEPROM: {err}");
                return;
            }
            Err(err) => {
                panic!("failed to parse the module EEPROM reply: {err:?}")
            }
        }
    }

    assert_eq!(msgs.len(), 1);
}
