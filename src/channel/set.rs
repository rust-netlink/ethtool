// SPDX-License-Identifier: MIT

use futures::TryStream;
use netlink_packet_generic::GenlMessage;

use crate::{ethtool_execute, EthtoolAttr, EthtoolError, EthtoolHandle, EthtoolMessage, EthtoolChannelAttr};

pub struct EthtoolChannelSetRequest {
    handle: EthtoolHandle,
    message: EthtoolMessage,
}

impl EthtoolChannelSetRequest {
    pub(crate) fn new(handle: EthtoolHandle, iface_name: &str) -> Self {
        EthtoolChannelSetRequest {
            handle,
            message: EthtoolMessage::new_channel_set(iface_name),
        }
    }

    pub fn rx_count(mut self, count: u32) -> Self {
        self.message.nlas.push(EthtoolAttr::Channel(EthtoolChannelAttr::RxCount(count)));
        self
    }

    pub fn tx_count(mut self, count: u32) -> Self {
        self.message.nlas.push(EthtoolAttr::Channel(EthtoolChannelAttr::TxCount(count)));
        self
    }

    pub fn other_count(mut self, count: u32) -> Self {
        self.message.nlas.push(EthtoolAttr::Channel(EthtoolChannelAttr::OtherCount(count)));
        self
    }

    pub fn combined_count(mut self, count: u32) -> Self {
        self.message.nlas.push(EthtoolAttr::Channel(EthtoolChannelAttr::CombinedCount(count)));
        self
    }

    pub async fn execute(
        self,
    ) -> impl TryStream<Ok = GenlMessage<EthtoolMessage>, Error = EthtoolError>
    {
        let EthtoolChannelSetRequest {
            mut handle,
            message,
        } = self;

        ethtool_execute(&mut handle, false, message).await
    }
}
