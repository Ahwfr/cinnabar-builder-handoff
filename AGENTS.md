# Continuing Cinnabar Builder

Read [docs/HANDOFF.md](docs/HANDOFF.md) and [docs/FORK.md](docs/FORK.md) before editing. Use [docs/CHAT-HISTORY.md](docs/CHAT-HISTORY.md) for earlier user feedback and screenshots described in the chat.

- First reproduce the unmodified pinned native build and run it with the isolated user root. Record tool versions, results, and the exact build command. Check for an existing checkout before downloading one.
- Keep Cinnabar client edits on a small `builder-features` branch. Keep the upstream base marked and the update instructions usable by a Windows user. Reuse the Cinnabar launcher.
- Start with textured opacity. Keep Creative Easy Place in a separate patch after opacity. Keep schematic import, UI, and per-blueprint policy in the guest where possible.
- Preserve explicit blueprint selection, current layer controls, material counts, and game input bindings. Test the same guest on stock Cinnabar after adding host capabilities.
- Keep the current component, importer, and editable source together for each release. Archive prior releases and preserve original schematic inputs. Keep user worlds, inputs, release binaries, and credentials out of Git.
