//! Prebuilt XeSS shared libraries
//!
//! This crate provides path accessors for the prebuilt XeSS shared libraries bundled in the XeSS
//! submodule. Applications use these paths to locate and copy the XeSS inference DLLs at build or
//! deploy time.
//!
//! # Warning: git/path dependency only
//!
//! This crate resolves the XeSS binaries via a relative path into the `xess/` submodule
//! (`CARGO_MANIFEST_DIR/../xess/bin`). This **only works as a git or path dependency** — it cannot
//! be published to crates.io (the binaries are 110 MiB+ of proprietary Intel blobs that exceed the
//! 10 MB crate size limit; their only alternative is downloading at build time from a public URL).

use std::path::{Path, PathBuf};

const XESS_BIN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../xess/bin");

/// Target platform for XeSS libraries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
}

impl Platform {
    /// Returns the platform matching the Cargo `CARGO_CFG_TARGET_OS` environment variable.
    ///
    /// Cargo sets this for build scripts. Panics outside that context or on
    /// unsupported targets.
    pub fn for_current_target() -> Self {
        let os = std::env::var("CARGO_CFG_TARGET_OS")
            .expect("CARGO_CFG_TARGET_OS not set (are you in a build script?)");
        match os.as_str() {
            "windows" => Self::Windows,
            _ => panic!("unsupported target OS {os:?} — only Windows has XeSS binaries"),
        }
    }
}

/// XeSS feature variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    /// XeSS Super Resolution.
    Xess,
    /// XeSS Super Resolution (D3D11).
    XessDx11,
    /// Xe Low-Latency.
    Xell,
    /// XeSS Frame Generation.
    XessFg,
}

/// Returns the path to a XeSS shared library for the given feature and platform.
pub fn dll_path(feature: Feature, platform: Platform) -> PathBuf {
    let xess_bin_dir = Path::new(XESS_BIN_DIR);

    assert!(
        xess_bin_dir.exists(),
        "Cannot find `{xess_bin_dir:?}`. You might be trying to reference `xess-bin` without it being a git/path dependency!"
    );

    let filename = match (feature, platform) {
        (Feature::Xess, Platform::Windows) => "libxess.dll",
        (Feature::XessDx11, Platform::Windows) => "libxess_dx11.dll",
        (Feature::Xell, Platform::Windows) => "libxell.dll",
        (Feature::XessFg, Platform::Windows) => "libxess_fg.dll",
    };

    xess_bin_dir.join(filename)
}
