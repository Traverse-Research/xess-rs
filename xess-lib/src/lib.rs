use std::path::{Path, PathBuf};

pub fn dll_path_xess() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("PrebuiltBinaries/libxess.dll")
}
pub fn dll_path_xess_fg() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("PrebuiltBinaries/libxess_fg.dll")
}
pub fn dll_path_xell() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("PrebuiltBinaries/libxell.dll")
}
pub fn dll_path_xess_dx11() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("PrebuiltBinaries/libxess_dx11.dll")
}
