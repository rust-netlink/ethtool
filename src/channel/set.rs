// SPDX-License-Identifier: MIT

use futures::TryStream;
use netlink_packet_generic::GenlMessage;

use crate::{
    ethtool_execute, EthtoolAttr, EthtoolChannelAttr, EthtoolError,
    EthtoolHandle, EthtoolMessage,
};

pub struct EthtoolChannelSetRequest {
    handle: EthtoolHandle,
    message: EthtoolMessage,
    rx_count: Option<u32>,
    tx_count: Option<u32>,
    other_count: Option<u32>,
    combined_count: Option<u32>,
}

impl EthtoolChannelSetRequest {
    pub(crate) fn new(handle: EthtoolHandle, iface_name: &str) -> Self {
        EthtoolChannelSetRequest {
            handle,
            message: EthtoolMessage::new_channel_set(iface_name),
            rx_count: None,
            tx_count: None,
            other_count: None,
            combined_count: None,
        }
    }

    pub fn rx_count(mut self, count: u32) -> Self {
        self.rx_count = Some(count);
        self
    }

    pub fn tx_count(mut self, count: u32) -> Self {
        self.tx_count = Some(count);
        self
    }

    pub fn other_count(mut self, count: u32) -> Self {
        self.other_count = Some(count);
        self
    }

    pub fn combined_count(mut self, count: u32) -> Self {
        self.combined_count = Some(count);
        self
    }

    pub async fn execute(
        self,
    ) -> impl TryStream<Ok = GenlMessage<EthtoolMessage>, Error = EthtoolError>
    {
        let EthtoolChannelSetRequest {
            mut handle,
            mut message,
            rx_count,
            tx_count,
            other_count,
            combined_count,
        } = self;

        if let Some(count) = rx_count {
            message
                .nlas
                .push(EthtoolAttr::Channel(EthtoolChannelAttr::RxCount(count)));
        }
        if let Some(count) = tx_count {
            message
                .nlas
                .push(EthtoolAttr::Channel(EthtoolChannelAttr::TxCount(count)));
        }
        if let Some(count) = other_count {
            message.nlas.push(EthtoolAttr::Channel(
                EthtoolChannelAttr::OtherCount(count),
            ));
        }
        if let Some(count) = combined_count {
            message.nlas.push(EthtoolAttr::Channel(
                EthtoolChannelAttr::CombinedCount(count),
            ));
        }

        ethtool_execute(&mut handle, false, message).await
    }
}
