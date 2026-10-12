// SPDX-License-Identifier: MIT

use futures_util::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;

use crate::{try_ethtool, EthtoolError, EthtoolHandle, EthtoolPauseParsed};

/// Apply a [EthtoolPauseParsed] configuration (equivalent to `ethtool -A
/// eth1 ...`).
///
/// Only fields holding `Some` are sent, the kernel keeps the current value
/// of the other pause properties.
pub struct EthtoolPauseSetRequest {
    handle: EthtoolHandle,
    config: EthtoolPauseParsed,
}

impl EthtoolPauseSetRequest {
    pub(crate) fn new(
        handle: EthtoolHandle,
        config: EthtoolPauseParsed,
    ) -> Self {
        EthtoolPauseSetRequest { handle, config }
    }

    pub async fn execute(self) -> Result<(), EthtoolError> {
        let EthtoolPauseSetRequest { mut handle, config } = self;

        let mut nl_msg =
            NetlinkMessage::from(GenlMessage::from_payload(config.build()?));

        nl_msg.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        let mut response = handle.request(nl_msg).await?;

        while let Some(message) = response.next().await {
            try_ethtool!(message);
        }

        Ok(())
    }
}
