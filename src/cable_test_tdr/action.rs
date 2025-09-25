// SPDX-License-Identifier: MIT

use futures::{future::Either, FutureExt, StreamExt, TryStream};
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;

use crate::{
    try_ethtool, EthtoolCablePair, EthtoolError, EthtoolHandle, EthtoolMessage,
};

/// Configuration for a TDR (Time Domain Reflectometry) cable test.
/// The ETHTOOL_A_CABLE_TEST_TDR_CFG is optional, as well as all members of the
/// nest. All distances are expressed in centimeters. The PHY takes the
/// distances as a guide, and rounds to the nearest distance it actually
/// supports.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EthtoolCableTestTdrConfig {
    /// The first measurement point in centimeters.
    pub first: Option<u32>,
    /// The last measurement point in centimeters.
    pub last: Option<u32>,
    /// The step size between measurement points in centimeters.
    pub step: Option<u32>,
    /// The cable pair to test.
    /// If a pair is passed, only that one pair will be tested. Otherwise all
    /// pairs are tested.
    pub pair: Option<EthtoolCablePair>,
}

pub struct EthtoolCableTestTdrActionRequest {
    handle: EthtoolHandle,
    iface_name: String,
    config: Option<EthtoolCableTestTdrConfig>,
}

impl EthtoolCableTestTdrActionRequest {
    pub(crate) fn new(
        handle: EthtoolHandle,
        iface_name: &str,
        config: Option<EthtoolCableTestTdrConfig>,
    ) -> Self {
        EthtoolCableTestTdrActionRequest {
            handle,
            iface_name: iface_name.to_string(),
            config,
        }
    }

    pub async fn execute(
        self,
    ) -> impl TryStream<Ok = GenlMessage<EthtoolMessage>, Error = EthtoolError>
    {
        let EthtoolCableTestTdrActionRequest {
            mut handle,
            iface_name,
            config,
        } = self;

        let ethtool_msg =
            EthtoolMessage::new_cable_test_tdr(&iface_name, config);
        let mut nl_msg =
            NetlinkMessage::from(GenlMessage::from_payload(ethtool_msg));

        // Use NLM_F_ACK because there is no REPLY for
        // ETHTOOL_MSG_CABLE_TEST_TDR_ACT.
        nl_msg.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        match handle.request(nl_msg).await {
            Ok(response) => {
                Either::Left(response.map(move |msg| Ok(try_ethtool!(msg))))
            }
            Err(e) => {
                Either::Right(
                    futures::future::err::<
                        GenlMessage<EthtoolMessage>,
                        EthtoolError,
                    >(e)
                    .into_stream(),
                )
            }
        }
    }
}
