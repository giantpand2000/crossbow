///! Commands for compiling rust code for `Android`,
///! generation/aligning/signing/installing/starting on device APKs and AAB,
///! generation `AndroidManifest.xml` and so on.
mod add_libs_into_aapt2;
mod add_libs_into_apk;
mod align_apk;
mod attach_logger;
mod detect_abi;
mod extract_apk;
mod gen_aab_from_modules;
mod gen_key;
mod gen_manifest;
mod gen_minimal_unsigned_aab;
mod gen_unaligned_apk;
mod gen_zip_modules;
mod install_apk;
mod read_manifest;
mod remove;
mod rust_compile;
mod save_manifest;
mod sign_apk;
mod start_apk;
mod write_zip;

pub use add_libs_into_aapt2::*;
pub use add_libs_into_apk::*;
pub use align_apk::*;
pub use attach_logger::*;
pub use detect_abi::*;
pub use extract_apk::*;
pub use gen_aab_from_modules::*;
pub use gen_key::*;
pub use gen_manifest::*;
pub use gen_minimal_unsigned_aab::*;
pub use gen_unaligned_apk::*;
pub use gen_zip_modules::*;
pub use install_apk::*;
pub use read_manifest::*;
pub use remove::*;
pub use rust_compile::*;
pub use save_manifest::*;
pub use sign_apk::*;
pub use start_apk::*;
pub use write_zip::*;
