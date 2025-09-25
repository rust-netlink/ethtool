// SPDX-License-Identifier: MIT

const ETHTOOL_A_CABLE_PAIR_A: u8 = 0;
const ETHTOOL_A_CABLE_PAIR_B: u8 = 1;
const ETHTOOL_A_CABLE_PAIR_C: u8 = 2;
const ETHTOOL_A_CABLE_PAIR_D: u8 = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EthtoolCablePair {
    A,
    B,
    C,
    D,
    Other(u8),
}

impl From<u8> for EthtoolCablePair {
    fn from(value: u8) -> Self {
        match value {
            ETHTOOL_A_CABLE_PAIR_A => Self::A,
            ETHTOOL_A_CABLE_PAIR_B => Self::B,
            ETHTOOL_A_CABLE_PAIR_C => Self::C,
            ETHTOOL_A_CABLE_PAIR_D => Self::D,
            _ => Self::Other(value),
        }
    }
}

impl From<EthtoolCablePair> for u8 {
    fn from(value: EthtoolCablePair) -> Self {
        match value {
            EthtoolCablePair::A => ETHTOOL_A_CABLE_PAIR_A,
            EthtoolCablePair::B => ETHTOOL_A_CABLE_PAIR_B,
            EthtoolCablePair::C => ETHTOOL_A_CABLE_PAIR_C,
            EthtoolCablePair::D => ETHTOOL_A_CABLE_PAIR_D,
            EthtoolCablePair::Other(v) => v,
        }
    }
}
