# Build and update the Cinnabar fork on Windows

The handoff repository stores the Schematica guest and instructions. The native branch lives in its own Cinnabar checkout. Cinnabar uses PolyForm Noncommercial 1.0.0. Keep the upstream license and third-party notices with native source and builds. [Pinned upstream source](https://github.com/cinnabar-mc/cinnabar/tree/dd7d7dd53a52c40491e0eb49b5bc700a4974191e)

## Get the starting checkout

From the handoff root, use PowerShell:

```powershell
.\scripts\Initialize-CinnabarBuilder.ps1
.\scripts\Start-CinnabarBuilder.ps1 -Check
```

If you already have the local Cinnabar checkout, skip initialization and pass its path with `-Checkout` to the launcher. Confirm `git status --short --branch`, `git rev-parse HEAD`, and `git log --oneline builder-base..builder-features` there. The original checkout was shallow and the fresh initializer uses a partial clone with branch history. For a shallow checkout, fetch the history with `git fetch --unshallow upstream dev` before the first rebase.

When publishing native commits, add the user's separate Cinnabar repository as `origin` and push `builder-features` and the base tag there explicitly. A public GitHub fork is a separate decision from the private handoff with chat history. Keep the chat in the handoff.

## Build and run

Install the build tools in the pinned upstream README and Makefile, including Go and GNU Make on PATH, and let rustup use the checkout's pinned Rust toolchain. The previous session observed missing Go and Make and did not complete a native build. Use the first build to resolve toolchain or dependency setup and record the working versions. [Cinnabar build instructions](https://github.com/cinnabar-mc/cinnabar/tree/dd7d7dd53a52c40491e0eb49b5bc700a4974191e#build-and-run)

```powershell
Push-Location .\work\cinnabar-builder
try { make play-build PROFILE=release } finally { Pop-Location }
.\scripts\Start-CinnabarBuilder.ps1 -Check
.\scripts\Start-CinnabarBuilder.ps1
```

The wrapper launches Cinnabar's own UI with an isolated `CINNABAR_USER_ROOT` under the checkout. Start with a disposable Creative test world. Import the WASM mod there and verify blueprint selection, rotation, single-layer display, material counts, and placement marker updates before changing the host.

## Update upstream

Keep the renderer, Creative placement and later editor changes in separate commits. Keep a known-good release and profile backup outside the build output. Run the following inside the native checkout with a clean working tree. Each Git command should succeed before you run the next one.

```powershell
git status --short --branch
git fetch upstream dev
git branch builder-backup-$(Get-Date -Format yyyyMMdd-HHmmss) builder-features
git switch -c builder-update builder-features
git rebase --onto upstream/dev builder-base
```

If rebase reports conflicts, use `git rebase --abort` to return the candidate to its starting state. Your active branch and base tag still point to the previous version. Keep the candidate branch for troubleshooting, or rename it before trying again.

If the rebase succeeds, build and run the candidate. Smoke test the stock launcher, world load, guest library, preview depth and opacity, and any Easy Place behavior already implemented. Keep an existing tested build available because the candidate build can overwrite files under `target/`. After the candidate passes:

```powershell
git branch -f builder-features builder-update
git tag -f builder-base upstream/dev
git switch builder-features
git branch -d builder-update
```

Before promotion, make sure `upstream/dev` is still the commit used for the rebase. Keep the timestamped backup branch. If the native branch is already published, either push the tested update under a new branch name, or coordinate history rewriting and use `git push --force-with-lease origin builder-features`. Update the remote base tag explicitly if you use it for other checkouts.
