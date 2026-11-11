// SPDX-License-Identifier: MIT

use futures_util::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;

use crate::{try_ethtool, EthtoolError, EthtoolHandle, EthtoolRingParsed};

/// Apply a [EthtoolRingParsed] configuration (equivalent to `ethtool -G
/// eth1 ...`).
///
/// Only fields holding `Some` are sent, the kernel keeps the current value
/// of the other ring parameters.
pub struct EthtoolRingSetRequest {
    handle: EthtoolHandle,
    config: EthtoolRingParsed,
}

impl EthtoolRingSetRequest {
    pub(crate) fn new(
        handle: EthtoolHandle,
        config: EthtoolRingParsed,
    ) -> Self {
        EthtoolRingSetRequest { handle, config }
    }

    pub async fn execute(self) -> Result<(), EthtoolError> {
        let EthtoolRingSetRequest { mut handle, config } = self;

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
