# Releasing Doc4Me

Doc4Me is built from this repository (`chatraweechai/doc4me`) by GitHub Actions. Today the
release pipeline builds the **Windows x64** installer; the packaging scripts for the other
platforms (inherited from WordCraft) are in [`packaging/`](../packaging/) and run locally.

1. Bump the version on `main`: `cargo xtask version set 1.0.1` (the one source of truth is
   `[workspace.package] version`; the app shows it in `--version` and About, with the commit from
   `DOC4ME_BUILD_SHA`).
2. Push to `main`. [`.github/workflows/windows-x64-installer.yml`](../.github/workflows/windows-x64-installer.yml)
   builds, on `windows-latest`:

| Platform | Artifacts | Signing |
|---|---|---|
| Windows | x64 `doc4me-<version>-windows-x64.msi` + `doc4me-<version>-windows-x64-portable.zip` (artifact `doc4me-windows-x64`) | none yet: `packaging/windows/sign.ps1` signs when its `AZURE_*` / `WINDOWS_CERTIFICATE*` secrets exist, and warns otherwise |

3. In [`CHANGELOG.md`](../CHANGELOG.md), move the entries under `## [Unreleased]` to a new
   `## [<version>] - YYYY-MM-DD` section, add its compare link at the bottom, and push to `main`.
4. To publish a GitHub Release, push a tag: `git tag v1.0.0 && git push origin v1.0.0`. The same
   workflow builds the tag and creates the release `Doc4Me v<version>` with the MSI and the zip;
   the release notes are that version's `CHANGELOG.md` section (GitHub's generated notes when the
   section is missing, with a warning in the run).

The workflow also runs on demand (Actions › Windows x64 installer › Run workflow). Its third-party
actions are pinned to commit SHAs; the build job only reads the repository, and only the tag's
release job may write (to create the release).

The MSI has its own `UpgradeCode` (`packaging/windows/doc4me.wxs`): it upgrades earlier Doc4Me
installs and never touches an installed WordCraft.

Local packaging: `packaging/windows/package.ps1 -Arch x64` (Windows, needs WiX v5),
`packaging/macos/package.sh --arch universal`, `packaging/linux/package.sh` (then
`packaging/linux/flatpak-bundle.sh` for the Flatpak), `packaging/web/package.sh`.
