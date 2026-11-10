# Cinnabar Builder handoff

This repository contains Schematica v4.2 for the Cinnabar Bedrock client and the plan for a small native Cinnabar fork. The WASM mod has blueprint import, an explicit library picker, movement, rotation, mirroring, layers, a material table, and mismatch indicators. The next milestone is a working Windows build of the pinned Cinnabar client, followed by translucent textured previews and Creative Easy Place.

- [AGENTS.md](AGENTS.md) gives the next assistant its starting instructions.
- [docs/HANDOFF.md](docs/HANDOFF.md) records the implementation state, entry points, requirements, and acceptance checks.
- [docs/FORK.md](docs/FORK.md) gives the Windows bootstrap, build, launch, and upstream update workflow.
- [mod/](mod/) contains the editable v4.2 Rust workspace, UI assets, WIT snapshot, and importer scripts.
- [tools/smoke/](tools/smoke/) contains the Wasmtime regression harness prototype from the prior work.
- [docs/CHAT-HISTORY.md](docs/CHAT-HISTORY.md) contains the visible project conversation with portable path redactions.

Give the next assistant this prompt:

> Read AGENTS.md and docs/HANDOFF.md. Continue from the pinned Cinnabar base and the v4.2 mod in this repository. Get the unmodified Windows build running in an isolated profile, then implement textured translucent ghosts as the first small native patch. Follow docs/FORK.md so I can apply future Cinnabar updates myself.

The mod source includes the upstream Cinnabar WIT license and the original Schematica MIT notice. The native client is fetched separately and retains its upstream license and third-party notices. Personal schematics, compiled releases, and client data stay in the original workspace.
