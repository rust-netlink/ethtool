// SPDX-License-Identifier: MIT

use futures_util::StreamExt;
use netlink_packet_core::{NetlinkMessage, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;

use crate::{
    header::ALTIFNAMSIZ, try_ethtool, EthtoolAttr, EthtoolBit, EthtoolBitset,
    EthtoolBitsetValue, EthtoolCmd, EthtoolError, EthtoolFeatureAttr,
    EthtoolFeatureGetRequest, EthtoolHandle, EthtoolHeader, EthtoolMessage,
};

/// Change the features of an interface (equivalent to `ethtool -K eth1 ...`).
///
/// Features are named as `ethtool -k` names them, for example
/// `tls-hw-tx-offload`. Features that are not requested keep their state.
pub struct EthtoolFeatureSetRequest {
    handle: EthtoolHandle,
    iface_name: String,
    features: Vec<(String, bool)>,
}

impl EthtoolFeatureSetRequest {
    pub(crate) fn new(handle: EthtoolHandle, iface_name: &str) -> Self {
        EthtoolFeatureSetRequest {
            handle,
            iface_name: iface_name.to_string(),
            features: Vec::new(),
        }
    }

    /// Request feature `name` to be turned on or off.
    pub fn feature(mut self, name: &str, enable: bool) -> Self {
        self.features.push((name.to_string(), enable));
        self
    }

    /// Send the request.
    ///
    /// The kernel only changes the features the driver lists as changeable
    /// (`ETHTOOL_A_FEATURES_HW`). To fail early on a typo or on a feature
    /// the device can not change, the changeable features are queried
    /// first, like `ethtool -K` does, and a feature that is not one of them
    /// fails with [EthtoolError::InvalidInput].
    ///
    /// The driver may still refuse a change it lists as possible, for
    /// example because the change depends on another feature. The returned
    /// bits are the requested features whose state differs from the request
    /// after the kernel applied it (the `ETHTOOL_A_FEATURES_WANTED` bitset of
    /// the reply), each holding the requested value. An empty list means
    /// every requested change took effect.
    pub async fn execute(self) -> Result<Vec<EthtoolBit>, EthtoolError> {
        let EthtoolFeatureSetRequest {
            mut handle,
            iface_name,
            features,
        } = self;

        let changeable = changeable_features(&handle, &iface_name).await?;
        let message = build_feature_set(&iface_name, &changeable, &features)?;

        let mut nl_msg =
            NetlinkMessage::from(GenlMessage::from_payload(message));
        nl_msg.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        let mut response = handle.request(nl_msg).await?;

        let mut unapplied = Vec::new();
        while let Some(message) = response.next().await {
            let reply = try_ethtool!(message).payload;
            unapplied.extend(feature_set_unapplied(reply)?);
        }
        Ok(unapplied)
    }
}

/// The `ETHTOOL_A_FEATURES_HW` bitset of `iface_name`: every feature with
/// its index, name and whether the user can change it.
async fn changeable_features(
    handle: &EthtoolHandle,
    iface_name: &str,
) -> Result<EthtoolBitset, EthtoolError> {
    let mut stream =
        EthtoolFeatureGetRequest::new(handle.clone(), Some(iface_name))
            .execute()
            .await?;
    let mut hw = None;
    while let Some(reply) = stream.next().await {
        for nla in reply?.payload.nlas {
            if let EthtoolAttr::Feature(EthtoolFeatureAttr::Hw(bitset)) = nla {
                hw = Some(bitset);
            }
        }
    }
    hw.ok_or_else(|| {
        EthtoolError::Bug(format!(
            "Got no ETHTOOL_A_FEATURES_HW from kernel for interface \
             `{iface_name}`"
        ))
    })
}

/// Build the `FEATURES_SET` [EthtoolMessage] requesting `features` on
/// `iface_name`, resolving each feature name against the `changeable`
/// bitset of the interface.
pub(crate) fn build_feature_set(
    iface_name: &str,
    changeable: &EthtoolBitset,
    features: &[(String, bool)],
) -> Result<EthtoolMessage, EthtoolError> {
    if iface_name.len() >= ALTIFNAMSIZ || iface_name.contains('\0') {
        return Err(EthtoolError::InvalidInput(format!(
            "`{iface_name}` is not a valid interface name"
        )));
    }
    let EthtoolBitsetValue::Bits(hw_bits) = &changeable.value else {
        return Err(EthtoolError::Bug(
            "Got compact ETHTOOL_A_FEATURES_HW bitset from kernel, which \
             carries no feature names"
                .to_string(),
        ));
    };
    let mut bits = Vec::with_capacity(features.len());
    for (name, enable) in features {
        // A bit is changeable when it is set, the parser marks every listed
        // bit of a bitset without mask as set.
        let changeable_bit = hw_bits
            .iter()
            .find(|bit| !name.is_empty() && bit.name == *name && bit.value);
        let Some(bit) = changeable_bit else {
            return Err(EthtoolError::InvalidInput(format!(
                "`{name}` is not a feature `{iface_name}` can change"
            )));
        };
        bits.push(EthtoolBit {
            index: bit.index,
            name: bit.name.clone(),
            value: *enable,
        });
    }

    Ok(EthtoolMessage {
        cmd: EthtoolCmd::FeatureSet,
        nlas: vec![
            EthtoolAttr::Feature(EthtoolFeatureAttr::Header(vec![
                EthtoolHeader::DevName(iface_name.to_string()),
            ])),
            // A bitset with mask, so the kernel changes the listed bits
            // only.
            EthtoolAttr::Feature(EthtoolFeatureAttr::Wanted(EthtoolBitset {
                nomask: false,
                size: changeable.size,
                value: EthtoolBitsetValue::Bits(bits),
            })),
        ],
    })
}

/// The requested features a `FEATURES_SET` reply says were not applied.
pub(crate) fn feature_set_unapplied(
    reply: EthtoolMessage,
) -> Result<Vec<EthtoolBit>, EthtoolError> {
    let mut unapplied = Vec::new();
    for nla in reply.nlas {
        if let EthtoolAttr::Feature(EthtoolFeatureAttr::Wanted(bitset)) = nla {
            match bitset.value {
                EthtoolBitsetValue::Bits(bits) => unapplied.extend(bits),
                EthtoolBitsetValue::Compact { .. } => {
                    return Err(EthtoolError::Bug(
                        "Got compact ETHTOOL_A_FEATURES_WANTED bitset from \
                         kernel, which was not requested"
                            .to_string(),
                    ))
                }
            }
        }
    }
    Ok(unapplied)
}
