# lib-ws-frame-handl

Pure-handl RFC 6455 frame codec — `encode`/`decode` for the `ws_frame` contract.

## Layout

```text
src/        the .hnl sources
pkg.hnl     package manifest — modules, deps, description, license
scripts/hnl-gate.sh   the gate
ci.sh       bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
