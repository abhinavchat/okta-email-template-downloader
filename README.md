# Okta Template Downloader

`okta-template-downloader` is a Rust CLI for downloading Okta email templates from the Admin APIs as:

1. `email.html`
2. `subject.txt`

It is designed for interactive use by non-technical users and supports:

1. Brand selection
2. Template selection or download-all
3. Current-directory or custom output location
4. Config via TOML or environment variables
5. Windows, macOS, and Linux release binaries

## What It Downloads

For each selected template, the CLI:

1. Tries to fetch brand-specific email customizations from Okta
2. Prefers the default customization when multiple customizations exist
3. Falls back to Okta `default-content` if no customization exists
4. Writes output as:
   `email.html`
   `subject.txt`

## Requirements

To run from source, you need:

1. Rust toolchain installed
2. Network access to your Okta org
3. An Okta API token with permission to read brands and email templates

End users of release binaries do not need Rust, Node, or any other runtime installed.

## Build From Source

From the repository root:

```bash
cargo build
```

The debug binary will be created at:

```text
target/debug/okta-template-downloader
```

On Windows:

```text
target\debug\okta-template-downloader.exe
```

For an optimized local build:

```bash
cargo build --release
```

## Configuration

The CLI supports:

1. CLI flags
2. Environment variables
3. TOML config files

Precedence:

1. CLI flags
2. Environment variables
3. TOML config

### Environment Variables

macOS/Linux:

```bash
export OKTA_DOMAIN="your-org.okta.com"
export OKTA_API_TOKEN="your-api-token"
export OKTA_OUTPUT_DIR="./downloads"
```

Windows PowerShell:

```powershell
$env:OKTA_DOMAIN="your-org.okta.com"
$env:OKTA_API_TOKEN="your-api-token"
$env:OKTA_OUTPUT_DIR=".\downloads"
```

### TOML Config Files

The CLI searches for config in this order:

1. `.okta-template-downloader.toml` in the current directory
2. `okta-template-downloader.toml` in the current directory
3. user config directory:
   macOS/Linux: `~/.config/okta-template-downloader/config.toml`
   Windows: `%APPDATA%\okta-template-downloader\config.toml`

Example TOML:

```toml
okta_domain = "your-org.okta.com"
api_token = "your-api-token"
output_dir = "./downloads"
```

## Running The CLI

Interactive mode:

```bash
cargo run
```

With verbose output:

```bash
cargo run -- --verbose
```

Using the built binary directly:

```bash
./target/debug/okta-template-downloader
```

On Windows:

```powershell
.\target\debug\okta-template-downloader.exe
```

## Interactive Flow

When run without enough flags, the CLI will:

1. Load configuration
2. Fetch brands
3. Ask you to select a brand
4. Fetch templates for that brand
5. Ask you to select one template or all templates
6. Ask for an output destination
7. Download and write files

If files already exist, the CLI will ask whether to:

1. Overwrite
2. Skip
3. Stop

## Useful CLI Flags

```bash
--config <path>
--domain <okta-domain>
--token <api-token>
--output <dir>
--brand <brand-id>
--template <template-name>
--all
--non-interactive
--overwrite
--verbose
```

## Usage Examples

Interactive run:

```bash
cargo run
```

Interactive run with verbose logging:

```bash
cargo run -- --verbose
```

Download one template non-interactively:

```bash
cargo run -- \
  --brand bnd123 \
  --template UserActivation \
  --output ./downloads \
  --overwrite
```

Download all templates for a brand:

```bash
cargo run -- \
  --brand bnd123 \
  --all \
  --output ./downloads \
  --overwrite
```

Use a specific config file:

```bash
cargo run -- --config ./okta-template-downloader.toml
```

## Output Layout

Output is written like this:

```text
<destination>/
  <brand-name-or-id>/
    <template-name>/
      email.html
      subject.txt
```

Example:

```text
tmp_export/
  carey-ext_default/
    UserActivation/
      email.html
      subject.txt
```

## Smoke Test Checklist

Use this when verifying a new environment:

1. Build the project:
   `cargo build`
2. Configure `OKTA_DOMAIN` and `OKTA_API_TOKEN`
3. Run:
   `cargo run -- --verbose`
4. Select one known customized template
5. Confirm the CLI logs either:
   `using brand customization ...`
   or
   `no customization found ... using default-content`
6. Confirm `subject.txt` and `email.html` match the API response you expect

## Okta API Endpoints Used

The CLI uses:

1. `GET /api/v1/brands`
2. `GET /api/v1/brands/{brandId}/templates/email`
3. `GET /api/v1/brands/{brandId}/templates/email/{templateName}/customizations`
4. `GET /api/v1/brands/{brandId}/templates/email/{templateName}/default-content`

## Development Commands

Format:

```bash
cargo fmt
```

Test:

```bash
cargo test
```

Build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

## CI And Releases

The repository already includes GitHub Actions workflows:

1. [ci.yml](.github/workflows/ci.yml)
2. [release.yml](.github/workflows/release.yml)

### CI

The CI workflow runs on:

1. pull requests
2. pushes to `main`

It executes:

```bash
cargo test --all-targets
```

across:

1. Ubuntu
2. macOS
3. Windows

### Release Publishing

The release workflow runs when you push a tag matching:

```text
v*
```

Examples:

1. `v0.1.0`
2. `v1.0.0`

When a tag is pushed, GitHub Actions will:

1. build platform-specific binaries
2. create or update the GitHub Release for that tag
3. upload binaries as release assets

Current release targets:

1. Linux x86_64
2. macOS x86_64
3. macOS aarch64
4. Windows x86_64

## How To Publish A Release

Recommended flow:

1. Make sure your branch is clean
2. Update version in [Cargo.toml](Cargo.toml)
3. Run:
   `cargo fmt`
4. Run:
   `cargo test`
5. Commit your changes
6. Create a tag
7. Push the commit and tag

Example:

```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

Once the tag is pushed:

1. open GitHub Actions
2. wait for the `Release` workflow to finish
3. open the GitHub Release for that tag
4. confirm the binaries were attached

## Suggested Release Checklist

Before tagging:

1. Confirm local tests pass
2. Confirm the CLI can download at least one known template from a real Okta org
3. Confirm README examples still match the CLI behavior
4. Confirm the version in `Cargo.toml` is correct

After publishing:

1. Download one release asset
2. Run it on the target OS
3. Confirm interactive mode works
4. Confirm output files are written correctly

## Security Notes

1. Never commit API tokens into the repository
2. Do not paste live SSWS tokens into issues, PRs, or chat
3. Rotate a token immediately if it is ever exposed
4. Prefer config files outside the repository if multiple people use the same machine

## Troubleshooting

### Only some templates appear

The CLI follows Okta pagination. If the template list still looks incomplete, rerun with:

```bash
cargo run -- --verbose
```

### A template downloads the default content instead of brand content

That means the CLI did not find a brand customization and fell back to `default-content`. Verify by calling:

```bash
curl --location 'https://<OKTA-DOMAIN>/api/v1/brands/<BRAND-ID>/templates/email/<TEMPLATE>/customizations' \
  --header 'Authorization: SSWS <API_TOKEN>' \
  --header 'Content-Type: application/json'
```

### Existing files cause a prompt

This is expected. Choose:

1. overwrite
2. skip
3. stop

Or bypass the prompt with:

```bash
--overwrite
```
