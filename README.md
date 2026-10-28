# lib-ws-frame-handl

Pure-handl RFC 6455 frame codec — `encode`/`decode` for the `ws_frame` contract.

Part of the `handl-lang` org split (ADR-0128). History is a `git subtree
split` of the platform monorepo's `ws_frame-handl/` (via `web`).

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
