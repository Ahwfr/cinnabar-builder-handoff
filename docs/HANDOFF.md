# Implementation handoff

## Where work stopped

- **WASM mod.** v4.2 is the latest editable source in `mod/`. The user confirmed that layers and materials worked in earlier builds. v4.2 refined menus and was checked against the offline layout preview. Live v4.2 menu and full client integration still need a smoke test.
- **Native client.** The original workspace has `work/cinnabar-builder`, a shallow checkout with branch `builder-features` and tag `builder-base`, both at upstream `dd7d7dd53a52c40491e0eb49b5bc700a4974191e` on `dev`. The tree was clean and had zero native feature commits. A wrapper was written and its preflight ran. Go and GNU Make were missing from PATH. A native Rust check ran out of cached dependencies at pinned `minidump-writer`. The subsequent fetch was stopped before the build completed.
- **Release layout on the original PC.** `outputs/Latest/` holds the v4.2 source, a source ZIP, the Windows importer ZIP, and a WASM library containing the demo arch and the user's passenger wagon. `outputs/Archive/` holds earlier releases. The repository has the editable source and can generate a clean demo. Preserve the original passenger-wagon input if testing on that PC.

## Current guest

| Area | Entry point |
| --- | --- |
| Blueprint model, import and translations | `mod/core/src/`, `mod/builder/src/` |
| WASM guest, staged loading and drawing | `mod/guest/src/` |
| Menus and generated JSON UI | `mod/guest/menu_layout.ps1`, `mod/guest/ui/` |
| Host interface snapshot and license | `mod/guest/wit/` |
| Installation, complete key list and build commands | [mod/README.md](../mod/README.md) |

Use Alt+J to open the library and load one selected blueprint. No blueprint is placed automatically. Alt+I opens manipulation and materials, Alt+K opens settings, Alt+U identifies the planned block under the crosshair, and Alt+P re-anchors the guide. Keep the current Alt shortcuts as the compatible default. The original Java shortcuts were tried and collided with Cinnabar or other mods. Both slash keys opened chat, the Numpad * UI was clipped, Shift+middle crouched, middle click picked the live-world block, and Right Shift opened another mod. Preserve the user’s preference for the original Schematica look and workflow when changing menus. A configurable Java-style key preset would need the conflicts addressed and tested first. [Original Java Schematica reference](https://github.com/Lunatrius/Schematica/tree/dfab015f8436d61f7a362052659eb5bfee9f4271)

The importer supports legacy Java `.schematic`, Sponge `.schem`, `.litematic`, and Bedrock `.mcstructure`. Per-blueprint limits are 64 per axis, 5,120 cells including air, and 128 palette entries. The Windows picker accepts up to eight inputs and packs one library. The game mod cannot browse arbitrary desktop files, so import is a separate Windows tool. Keep the selected library item distinct from the active world guide and release old draw handles when switching.

The wagon originally trapped on its first frame with **all fuel consumed by WebAssembly** during the material tally. v3 staged decode, material grouping, palette resolution, and terrain and mesh work. The exact wagon and a dense 5,120-cell mock regression passed on v3, with peaks around 188k and 301k against the harness's 1M frame fuel budget. Re-run those checks when changing guest staging, especially on the integrated fork. v4.2 previously passed the 18 core tests. Windows Application Control blocked launching some newly compiled importer and harness binaries. Guest compilation still worked. On the original PC an existing v4 codec could prepare `.scb` data and pack the new guest. Use a trusted supported build environment if policy blocks local native binaries.

During handoff packaging, the copied mod files matched the release bytes and the launcher preflight ran. The fresh core test executable and native build scripts needed for a clean WASM build were blocked by Windows Application Control. A workspace check against the earlier target cache also failed while loading `phf_macros`. Revalidate on a clean supported build machine.

[tools/smoke/](../tools/smoke/) preserves the prototype library harness. It expects a packed component containing the demo at index 0 and the original wagon at index 1. The wagon itself stays local. For a portable test, pack a synthetic second blueprint and adapt the expected name and counts. Review mock API signatures and UI event IDs against the current guest before using the harness as a release gate.

To generate a clean demo, run `rustup target add wasm32-unknown-unknown` and then `./Build-Schematica.ps1` from `mod/`. For a user schematic, pass the input path and an optional `-Output` as documented in the mod README. For the Windows picker, build the native `schematica-builder` and the release `schematica-guest` for `wasm32-unknown-unknown`. Package the builder as `Schematica-Codec.exe` and the core guest as `schematica-guest-template.bin` beside the two importer scripts. The builder's CLI entry point is `mod/builder/src/main.rs`.

## Patch 1 · Translucent textured preview

The user wants the real texture and model of each missing block with modest transparency. Keep wrong and extra markers separate. The stock textured path uses native opacity, and the cyan and white cube variants do not satisfy the request.

1. Trace retained block-grid submission through `crates/mod-host/src/drawing.rs`, `crates/mod-render/src/retained/validation.rs`, and `crates/render/src/chunk/blocks.rs` plus its `blocks/` modules. Start from the pinned source when checking path names and ABI details.
2. Introduce a bounded, opt-in opacity through a compatible host capability. Normal grids retain their original rendering path. Reuse native textured meshes, texture alpha and cutouts, and the world depth test. Send the ghost to the appropriate alpha-blended pass, avoiding opaque depth writes. Check sorting when transparent geometry overlaps.
3. Route missing-block preview through that capability in the guest. Start with a modest default such as 65% opacity and tune it in-game with the user. Preserve the stock-host fallback, visibility toggle, wrong/extra markers, and layer filtering.
4. Smoke test with stone, cutout foliage, glass, stairs, a foreground wall, and overlapping ghosts. Verify resource packs still supply the texture and correctly placed blocks clear their ghost. Verify normal world and stock guest rendering.

## Patch 2 · Creative Easy Place

When enabled in Creative, one physical right-click aimed at a reachable missing schematic cell should choose its matching Creative item and place one block. Aim can come from the ghost, subject to reach, line of sight, a legal support face, and standard placement checks. If a specific block state cannot be placed safely, report it and leave the world alone. With the toggle off or outside Creative, preserve ordinary use. The existing **Target info** feature identifies blocks and coordinates only.

1. Trace `app/src/modding/inventory.rs`, `crates/client-ui/src/ui_runtime/mod_inventory.rs`, the native `app/src/pick_block`, `hotbar`, and `block_use` paths, and `crates/gameplay/src/placement_prediction.rs`. The guest's `creative-take` targets an empty inventory slot or cursor. Its `pulse-use` uses the selected item and current target while use is held. These APIs do not form an atomic directed placement.
2. Keep blueprint lookup and UI policy in the guest. Expose a narrow Creative-only host request and use the inventory ledger and standard item-use pipeline. Resolve the expected canonical block state and item metadata, find a legal face, and predict the result. Hold one pending request per input edge. Revalidate mode, focus, world, target, reach, and selected item after inventory acknowledgement. Consume the handled click once and prevent a duplicate ordinary use.
3. Exercise one click and held right-click, an empty hand and a populated hotbar, a simple cube and oriented stair, a wrong or occupied target, no support face, reach and occlusion, inventory or server rejection, disconnect or world change, GUI focus, and Survival refusal. Verify the placed result with normal terrain reads and the material table.

## Later scope

Use the same volume and ghost drawing for Axiom-style selection, clipboard and paste preview. Consider authorized local or cooperating-server edits with bounded undo after those tools work. Keep launcher and rendering changes independent of bulk editing. [Axiom reference](https://axiomdocs.moulberry.com/)
