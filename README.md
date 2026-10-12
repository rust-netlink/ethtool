# Rust crate for linux ethtool netlink management

This crate provides methods to query and configure the Linux network
interface settings via the ethtool generic netlink protocol.

The examples are built with the crate and need root (`CAP_NET_ADMIN`) to run:

```bash
cargo build --examples
sudo ./target/debug/examples/get_pause_conf
```

## Query

- [get or dump pause configuration](examples/get_pause_conf.rs)
- [query features](examples/query_features.rs)
- [query link modes](examples/query_link_mode.rs)
- [query rings](examples/query_rings.rs)
- [query coalesce](examples/query_coalesce.rs)
- [query timestamping information](examples/query_tsinfo.rs)
- [query FEC](examples/query_fec.rs)
- [query channels](examples/query_channels.rs)
- [query module EEPROM](examples/query_eeprom.rs)

## Set

- [apply pause configuration](examples/apply_pause_conf.rs)
- [set channel count](examples/set_channel_count.rs)

## Notes

- `EthtoolHandle::get_pause()` takes a mandatory interface name and returns
  a single `EthtoolPauseParsed`, `EthtoolHandle::dump_pause()` takes no
  interface name and returns a `Vec<EthtoolPauseParsed>`.
- `EthtoolHandle::apply_pause()` applies an `EthtoolPauseParsed`, usually
  one returned by `get_pause()` or `dump_pause()` after modifying it. The
  sub-handle request builders (`handle.pause().get(...)`,
  `handle.pause().set(...)`) stay available as well.
- `EthtoolPauseParsed` is `#[non_exhaustive]`, so it can not be built with
  a struct literal outside of this crate. Use `Default::default()` and
  assign the properties instead.
- A parsed configuration holds only the properties the kernel reported, a
  field holding `None` is not sent in a set request and the kernel keeps
  its current value.
- The statistics properties of `EthtoolPauseParsed` are read-only, the
  kernel rejects them in a set request, therefore they are not sent.
- `apply_pause_conf` shows how a queried configuration is modified and
  applied back, `set_channel_count` uses the builder of the channel set
  request.
