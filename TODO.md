# TODO

## Bugs found with nlmon captured packets (fixed)

- `EthtoolPauseStatAttr::Tx` emitted `ETHTOOL_A_PAUSE_STAT_RX_FRAMES`
  instead of `ETHTOOL_A_PAUSE_STAT_TX_FRAMES` (`src/pause/attr.rs`).
- `EthtoolLinkModeAttr::Ours`/`Peer`, `EthtoolTsInfoAttr` bitsets and
  `EthtoolFecAttr::Modes`/`Stats` were emitted without the
  `NLA_F_NESTED` flag the kernel sets.
- Feature verbose bitset parsing dropped index 0 and name-less bits and
  ignored `ETHTOOL_A_BITSET_SIZE`/`ETHTOOL_A_BITSET_NOMASK`, so the
  kernel reply could not be emitted back byte for byte.
- Compact bitsets (`ETHTOOL_FLAG_COMPACT_BITSETS`, used by `ethtool -k`)
  failed to parse with "No ETHTOOL_A_BITSET_BITS NLA found".
- FEC stats failed to parse when the driver has no corrected bits stats
  (empty `ETHTOOL_A_FEC_STAT_CORR_BITS`) or reports per lane values.
- Emitting parsed bitsets panicked with `todo!()` for feature, link mode,
  timestamping and FEC messages.

## unit tests

- Module EEPROM (`ethtool -m`) reply parsing has no captured-packet
  test. Requires SFP hardware which none of the virtual NICs
  (virtio-net, veth, netdevsim) provide.
- `EthtoolTsInfoAttr::TxTypes`/`RxFilters` parse paths are not covered,
  virtio-net, veth and netdevsim do not report tx types/rx filters.
- Non-empty `EthtoolLinkModeAttr::Ours`/`Peer` bitsets are not covered,
  none of the available virtual NICs reports supported link modes.
- `EthtoolPauseStatAttr::TxPauseStormEvents` parse path is not covered,
  netdevsim does not report pause storm events.

## API review

- Review whether public types need `#[non_exhaustive]`. Candidates:
  `EthtoolCmd` (kernel keeps adding commands), `EthtoolBitsetValue`
  (two UAPI forms today). Attribute enums already have
  `Other(DefaultNla)` catch-alls. `EthtoolBit`/`EthtoolBitset` have
  public fields users need to construct for set requests, adding
  `#[non_exhaustive]` there would block struct literals.

## attributes kept as `DefaultNla`

- `link_mode`: `ETHTOOL_A_LINKMODES_RATE_MATCHING` (10).
- `rings`: `ETHTOOL_A_RINGS_RX_PUSH` (14),
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN` (15),
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN_MAX` (16),
  `ETHTOOL_A_RINGS_HDS_THRESH` (17),
  `ETHTOOL_A_RINGS_HDS_THRESH_MAX` (18).
- `tsinfo`: `ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER` (7),
  `ETHTOOL_A_TSINFO_HWTSTAMP_SOURCE` (8),
  `ETHTOOL_A_TSINFO_HWTSTAMP_PHYINDEX` (9).
- `EthtoolModuleEEPROMAttr` only parses `HEADER` and `DATA`,
  `OFFSET`/`LENGTH`/`PAGE`/`BANK`/`I2C_ADDRESS` fall through to
  `DefaultNla` (they are only used in requests today).

## apply support

`pause` is the pilot for the parsed/apply design:

- A get request executes into parsed types (`EthtoolPauseParsed`): the flat
  `EthtoolHandle::get_pause()` takes a mandatory interface name and returns
  one parsed configuration, `EthtoolHandle::dump_pause()` returns all of
  them. `EthtoolHandle::apply_pause()` applies a parsed type back with a
  set request. The old sub-handle request builders stay available.
- `build()` of the parsed type emits only fields holding `Some` (the
  kernel keeps the value of the others) and validates the input, failing
  with `EthtoolError::InvalidInput`.
- Roll out to the commands with kernel `*_SET` support not covered yet:
  `link_mode` (`ADVERTISING`), `feature` (`WANTED`), `ring`, `coalesce`,
  `fec` (`MODES`), `tsinfo`, EEE.
- `EthtoolError` could hold `{kind, msg, netlink_msg}` like
  `WireguardError` instead of the current enum.

## missing support

- `EthtoolMessage` parses a subset of ethtool commands only: `STATS_GET`,
  `MODULE_GET`, `EEE_GET`, `PHY_GET`, `TSCONFIG_GET`, ... return
  "Unsupported ethtool reply command".
- Compact bitsets keep the raw `value`/`mask` words, there is no bit
  name table to resolve the enabled features from a compact reply.
- Unknown attributes nested inside a bitset are dropped with a warning,
  a later kernel addition would break byte-exact round-trip of parsed
  messages.
