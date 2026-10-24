# Releasing Doc4Me

Doc4Me is built from this repository (`chatraweechai/doc4me`) by GitHub Actions. Today the
release pipeline builds the **Windows x64** installer; the packaging scripts for the other
platforms (inherited from WordCraft) are in [`packaging/`](../packaging/) and run locally.

1. Bump the version on `main`: `cargo xtask version set x.y.z` (the one source of truth is
   `[workspace.package] version`; the app shows it in `--version` and About, with the commit from
   `DOC4ME_BUILD_SHA`).
2. Push to `main`. [`.github/workflows/windows-x64-installer.yml`](../.github/workflows/windows-x64-installer.yml)
   builds, on `windows-latest`:

| Platform | Artifacts | Signing |
|---|---|---|
| Windows | x64 `doc4me-<version>-windows-x64.msi` + `doc4me-<version>-windows-x64-portable.zip` (artifact `doc4me-windows-x64`) | `packaging/windows/sign.ps1` signs `doc4me.exe`, `doc4me-cli.exe` and the MSI when the signing secrets below are set, and warns "leaving unsigned" otherwise (no secrets are set yet) |

   The binaries are built in a step without the signing secrets (`package.ps1 -BuildOnly`), so no
   dependency's build script or proc-macro can read them; the next step packages and signs
   (`package.ps1 -SkipBuild`). The secrets (Settings › Secrets and variables › Actions › New
   repository secret), either set:
   - a certificate file: `WINDOWS_CERTIFICATE` (the .pfx, base64) and `WINDOWS_CERTIFICATE_PASSWORD`.
     Code-signing certificates issued since 2023-06-01 keep their key in hardware (a token or an
     HSM), so most can't be exported as a .pfx; ask the vendor for cloud signing that CI can use.
   - Azure Trusted Signing (all six): `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`,
     `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE`.

   A secret that isn't set reads as empty, which `sign.ps1` treats as unset. After adding them, run
   the workflow on `main` (without **release**) and check the log has no "leaving unsigned" warning
   and the files carry a signature.
3. In [`CHANGELOG.md`](../CHANGELOG.md), move the entries under `## [Unreleased]` to a new
   `## [x.y.z] - YYYY-MM-DD` section; at the bottom add `[x.y.z]: …/compare/v<previous>...vx.y.z`
   and change `[Unreleased]` to `…/compare/vx.y.z...HEAD`. Run the checks CI doesn't:
   `cargo xtask ci`, the interface-text audit that is `#[ignore]`d in normal runs
   (`cargo test -p doc4me-ui-egui i18n -- --include-ignored`) and `cargo audit`. Push to `main`.
4. To publish a GitHub Release, push an annotated tag:
   `git tag -a vx.y.z -m "Doc4Me x.y.z" && git push origin vx.y.z`. The same
   workflow builds the tag and creates the release `Doc4Me v<version>` with the MSI and the zip;
   the release notes are that version's `CHANGELOG.md` section (GitHub's generated notes when the
   section is missing, with a warning in the run).
   - Without a tag push (a cloud session's git proxy refuses tags): Actions › Windows x64
     installer › Run workflow on `main` with **release** ticked. The release job creates the tag
     `v<version in Cargo.toml>` at the commit it built, with the release. It stops if that tag
     already exists, so bump the version first. Anyone who can run workflows here can release this
     way. A session can start it with the GitHub tools (`actions_run_trigger`, workflow
     `windows-x64-installer.yml`, ref `main`, input `release: true`).
   - Don't push to `main` until the release job has finished, above all not a change to
     `.github/workflows`. GitHub refuses a release whose target commit adds or changes workflow
     files compared with the default branch unless the token may modify workflows, and
     `GITHUB_TOKEN` can't be given that
     ([Create a release](https://docs.github.com/en/rest/releases/releases#create-a-release); the
     [2023 changelog](https://github.blog/changelog/2023-11-02-github-actions-enforcing-workflow-scope-when-creating-a-release/)
     words it as a workflow-changing commit that no branch or tag points at). Run 39783304886
     failed this way, `HTTP 403: Resource not accessible by integration`, after `e06edac` (a
     workflow change) landed on `main` mid-run. Nothing is tagged or released then; start the run
     again on the new `main`.

The workflow also runs on demand (Actions › Windows x64 installer › Run workflow). Its third-party
actions are pinned to commit SHAs; the build job only reads the repository, and only the release
job may write (`contents: write`, to create the tag and the release).

The MSI has its own `UpgradeCode` (`packaging/windows/doc4me.wxs`): it upgrades earlier Doc4Me
installs and never touches an installed WordCraft.

Local packaging: `packaging/windows/package.ps1 -Arch x64` (Windows, needs WiX v5),
`packaging/macos/package.sh --arch universal`, `packaging/linux/package.sh` (then
`packaging/linux/flatpak-bundle.sh` for the Flatpak), `packaging/web/package.sh`.
