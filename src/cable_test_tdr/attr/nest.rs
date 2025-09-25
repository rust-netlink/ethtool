// SPDX-License-Identifier: MIT

use netlink_packet_core::{
    DecodeError, DefaultNla, Emitable, ErrorContext, Nla, NlaBuffer,
    NlasIterator, Parseable, NLA_F_NESTED,
};

use crate::cable_test_tdr::attr::{
    amplitude::EthtoolCableTestTdrAmplitudeAttr,
    pulse::EthtoolCableTestTdrPulseAttr, step::EthtoolCableTestTdrStepAttr,
};

const ETHTOOL_A_CABLE_TDR_NEST_STEP: u16 = 1;
const ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE: u16 = 2;
const ETHTOOL_A_CABLE_TDR_NEST_PULSE: u16 = 3;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EthtoolCableTestTdrNestAttr {
    Step(Vec<EthtoolCableTestTdrStepAttr>),
    Amplitude(Vec<EthtoolCableTestTdrAmplitudeAttr>),
    Pulse(Vec<EthtoolCableTestTdrPulseAttr>),
    Other(DefaultNla),
}

impl Nla for EthtoolCableTestTdrNestAttr {
    fn value_len(&self) -> usize {
        match self {
            Self::Step(step) => step.as_slice().buffer_len(),
            Self::Amplitude(amplitude) => amplitude.as_slice().buffer_len(),
            Self::Pulse(pulse) => pulse.as_slice().buffer_len(),
            Self::Other(attr) => attr.value_len(),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Step(_) => ETHTOOL_A_CABLE_TDR_NEST_STEP | NLA_F_NESTED,
            Self::Amplitude(_) => {
                ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE | NLA_F_NESTED
            }
            Self::Pulse(_) => ETHTOOL_A_CABLE_TDR_NEST_PULSE | NLA_F_NESTED,
            Self::Other(attr) => attr.kind(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Step(step) => step.as_slice().emit(buffer),
            Self::Amplitude(amplitude) => amplitude.as_slice().emit(buffer),
            Self::Pulse(pulse) => pulse.as_slice().emit(buffer),
            Self::Other(attr) => attr.emit(buffer),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for EthtoolCableTestTdrNestAttr
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        match buf.kind() & !NLA_F_NESTED {
            ETHTOOL_A_CABLE_TDR_NEST_STEP => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolCableTestTdrStepAttr::parse(&nla?).context(
                            "failed to parse ETHTOOL_A_CABLE_TDR_NEST_STEP",
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Step(nlas))
            }
            ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolCableTestTdrAmplitudeAttr::parse(&nla?)
                            .context("failed to parse ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE")
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Amplitude(nlas))
            }
            ETHTOOL_A_CABLE_TDR_NEST_PULSE => {
                let nlas = NlasIterator::new(buf.value())
                    .map(|nla| {
                        EthtoolCableTestTdrPulseAttr::parse(&nla?).context(
                            "failed to parse ETHTOOL_A_CABLE_TDR_NEST_PULSE",
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self::Pulse(nlas))
            }
            _ => {
                let other = DefaultNla::parse(buf)
                    .context("failed to parse unknown NLA for TDR nest")?;
                Ok(Self::Other(other))
            }
        }
    }
}
