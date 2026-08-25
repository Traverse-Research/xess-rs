# Prebuilt XeSS shared libraries

This crate provides path accessors for the prebuilt XeSS shared libraries bundled in the XeSS submodule. Applications use these paths to locate and copy the inference DLLs at build or deploy time.

## Warning: git/path dependency only

This crate resolves the binaries via a relative path into the `xess/` submodule (`CARGO_MANIFEST_DIR/../xess/bin`). This **only works as a git or path dependency** — it cannot be published to crates.io (the binaries are 110 MiB+ of proprietary Intel blobs that exceed the 10 MB crate size limit; their only alternative is downloading at build time from a public URL).

