use std::path::{Path, PathBuf};

pub fn dll_path_xess() -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("PrebuiltBinaries/libxess.dll")
}
pub fn dll_path_xess_fg() -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("PrebuiltBinaries/libxess_fg.dll")
}
pub fn dll_path_xell() -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("PrebuiltBinaries/libxell.dll")
}
pub fn dll_path_xess_dx11() -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("PrebuiltBinaries/libxess_dx11.dll")
}
