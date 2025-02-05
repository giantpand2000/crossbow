use super::*;
use clap::Parser;
use crossbundle_tools::utils::Config;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug, Default)]
pub struct BundletoolInstallCommand {
    /// Required. Version of download bundletool. For example:
    /// --version 1.8.2
    #[clap(long, short, default_value = "1.8.2")]
    version: String,
    /// Path to install bundletool. By default bundletool will be downloaded and saved in home directory
    #[clap(long, short)]
    path: Option<PathBuf>,
    /// Force install bundletool even if found.
    #[clap(long, short)]
    force: bool,
}

impl BundletoolInstallCommand {
    /// Download and install bundletool to provided or default path
    pub fn install(&self, config: &Config) -> crate::error::Result<()> {
        config.status("Installing bundletool")?;
        if self.force == false {
            for bundletool in
                std::fs::read_dir(default_file_path(self.file_name())?.parent().unwrap())?
            {
                let installed_bundletool = bundletool?.path();
                if installed_bundletool.ends_with(Self::file_name(&self)) {
                    config.status("You have installed budletool on your system already")?;
                    return Ok(());
                }
            }
        }
        let download_url = std::path::Path::new(super::BUNDLETOOL_JAR_FILE_DOWNLOAD_URL)
            .join(self.version.clone())
            .join(self.file_name());
        let download_url_str = String::from(download_url.to_str().unwrap());

        if let Some(install_path) = &self.path {
            let jar_path = install_path.join(self.file_name());
            download_to_file(&download_url_str, &jar_path)?;
        } else {
            let default_jar_path = default_file_path(self.file_name())?;
            download_to_file(&download_url_str, &default_jar_path)?;
        };
        config.status("Bundletool was installed successfully")?;
        Ok(())
    }

    /// Return bundletool jar file name with specified version
    fn file_name(&self) -> String {
        format!("bundletool-all-{}.jar", self.version)
    }
}
