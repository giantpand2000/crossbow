use crate::*;
use clap::Clap;
use creator_tools::types::*;
use creator_tools::*;

#[derive(Clap)]
pub struct AppleBuildCommand {
    /// Build profile. Can be one of: debug, release
    #[clap(short, long, default_value = "debug")]
    pub profile: Profile,
}

impl AppleBuildCommand {
    pub fn run(&self) -> Result<()> {
        let path = std::env::current_dir().unwrap();
        let target_dir = path.join("..").join("..").join("target");
        let manifest = Manifest::from_path_with_metadata(path.join("Cargo.toml"))?;
        let metadata = manifest.package.unwrap().metadata.unwrap().apple;
        let properties = &metadata.info_plist;
        let name = properties.launch.bundle_executable.clone().unwrap();
        let bundle_id = &properties.identification.bundle_identifier;
        // Compile app
        let build_target = metadata.build_targets.unwrap()[0];
        apple_rust_compile(&name, build_target, &path, self.profile, vec![]).unwrap();
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(&self.profile);
        let bin_path = out_dir.join(&name);
        assert!(bin_path.exists());
        // Generate app folder
        let app_dir = gen_apple_app(
            &target_dir,
            &name,
            Some(path.join(metadata.resources.unwrap())),
            Some(path.join(metadata.assets.unwrap())),
        )
        .unwrap();
        assert!(app_dir.exists());
        // Copy binary to app folder
        std::fs::copy(&bin_path, &app_dir.join(&name)).unwrap();
        // Generate Info.plist
        gen_apple_plist(&app_dir, properties, false).unwrap();
        // Install and launch on simulator
        let _device = launch_apple_app(&app_dir, "iPhone 8", bundle_id, false).unwrap();
        // device.shutdown().unwrap();
        creator_tools::simctl::Simctl::new().open().unwrap();
        Ok(())
    }
}