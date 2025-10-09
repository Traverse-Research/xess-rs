#[cfg(feature = "generate-bindings")]
fn vulkan_sdk_include_directory() -> Option<std::path::PathBuf> {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let is_windows = target_os.as_str() == "windows";

    // Mostly on Windows, the Vulkan headers don't exist in a common location but can be found based
    // on VULKAN_SDK, set by the Vulkan SDK installer.
    match std::env::var("VULKAN_SDK") {
        Ok(v) => Some(std::path::PathBuf::from(v).join(
            // On the Windows SDK the `Include` directory is capitalized
            if is_windows { "Include" } else { "include" },
        )),
        // TODO: On Windows, perhaps this should be an error with a link to the SDK installation?
        Err(std::env::VarError::NotPresent) if is_windows => {
            // On Windows there's no common include directory like `/usr/include` where Vulkan headers can be found
            panic!("When targeting Windows, the VULKAN_SDK environment variable must be set")
        }
        Err(std::env::VarError::NotPresent) => None,
        Err(std::env::VarError::NotUnicode(e)) => {
            panic!("VULKAN_SDK environment variable is not Unicode: {e:?}")
        }
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    let compile = |input_file, output_file, allowlist_function, allowlist_type| {
        let mut bindings = bindgen::Builder::default()
            .header(input_file)
            .allowlist_recursively(false)
            .allowlist_function(allowlist_function)
            .allowlist_type(allowlist_type)
            .allowlist_var(".*xess.*")
            .bitfield_enum(".*(flags|bits).*")
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: false,
            })
            .parse_callbacks(Box::new(RenameCallback))
            .derive_default(true)
            .clang_args(["-x", "c++"])
            .prepend_enum_name(false)
            .layout_tests(false)
            .dynamic_link_require_all(true)
            .dynamic_library_name("XessLoaded");
        if let Some(vulkan_sdk_include_dir) = vulkan_sdk_include_directory() {
            bindings = bindings.clang_arg(format!("-I{}", vulkan_sdk_include_dir.display()))
        }
        bindings
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(output_file)
            .expect("Couldn't write bindings");
    };

    compile(
        "./xess/inc/xell/xell.h",
        "./src/xell/xell.rs",
        ".*xell.*",
        ".*xell.*",
    );

    compile(
        "./xess/inc/xell/xell_d3d12.h",
        "./src/xell/dx12.rs",
        ".*xellD3D12.*",
        ".*xess_d3d12.*",
    );

    compile(
        "./xess/inc/xess/xess_d3d11.h",
        "./src/xess/dx11.rs",
        ".*xessD3D11.*",
        ".*xess_d3d11.*",
    );

    compile(
        "./src/xess/dx12_include.h",
        "./src/xess/dx12.rs",
        ".*xessD3D12.*",
        ".*xess_(d3d12|resources_to_dump_t).*", // Inconsistent naming yuck
    );

    // xess_debug.h includes xess.h, so we simply use the debug version and get all bindings in one go.
    compile(
        "./xess/inc/xess/xess_debug.h",
        "./src/xess/xess.rs",
        ".*xess.*",
        ".*xess.*",
    );

    // Same here, xess_vk_debug.h includes xess_vk.h.
    compile(
        "./xess/inc/xess/xess_vk_debug.h",
        "./src/xess/vk.rs",
        ".*xessVK.*",
        ".*xess_vk.*",
    );

    compile(
        "./xess/inc/xess_fg/xefg_swapchain_d3d12.h",
        "./src/xefg_swapchain/dx12.rs",
        ".*xefgSwapChainD3D12.*",
        ".*xefg_swapchain_d3d12.*",
    );

    // Same here, xefg_swapchain_debug.h includes xefg_swapchain.h.
    compile(
        "./xess/inc/xess_fg/xefg_swapchain_debug.h",
        "./src/xefg_swapchain/swapchain.rs",
        ".*xefgSwapChain.*",
        ".*xefg_swapchain.*",
    );
}

#[cfg(feature = "generate-bindings")]
#[derive(Debug)]
struct RenameCallback;

#[cfg(feature = "generate-bindings")]
impl bindgen::callbacks::ParseCallbacks for RenameCallback {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        if let Some(enum_name) = enum_name {
            if enum_name.starts_with("_xess") {
                return original_variant_name
                    .strip_prefix("XESS_")
                    .map(|s| s.to_string());
            }
            if enum_name.starts_with("_xell") {
                return original_variant_name
                    .strip_prefix("XELL_")
                    .map(|s| s.to_string());
            }
            if enum_name.starts_with("_xefg_swapchain") {
                return original_variant_name
                    .strip_prefix("XEFG_SWAPCHAIN_")
                    .map(|s| s.to_string());
            }
        }
        None
    }
}

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
}
