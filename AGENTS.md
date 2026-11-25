# lib-ws-frame-handl

Pure-handl RFC 6455 frame codec — `encode`/`decode` for the
`ws_frame` contract (`lib-ws-frame`).

## Layout

| path | role |
| --- | --- |
| `src/frame.hnl` | the codec implementation |
| `src/frame.test.hnl` | the .hnl test suite |
| `pkg.hnl` | manifest — `modules: ["frame"]`, deps `ws_frame`, `math`, `option` |
| `scripts/hnl-gate.sh`, `workflows/gate.yaml` | the gate / bupha entry |

## Contract

- Pure-handl reference implementation of the `ws_frame` contract —
  RFC 6455 frame encode/decode.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
