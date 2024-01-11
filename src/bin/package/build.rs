mod deb;
mod rpm;
mod src;


use crate::package_error::PackageError;
use dboeger1_dotfiles::*;
use deb::build_deb;
use rpm::build_rpm;
use src::build_src;


pub(crate) fn build() -> Result<(), PackageError> {
    if PROJECT_PACKAGES_DIR.exists() {
        return Err(PackageError {
            message: format!(
                "cannot overwrite existing packages directory \"{}\"",
                PROJECT_PACKAGES_DIR.to_string_lossy(),
            ),
            source: None,
        });
    }

    build_src()?;
    build_deb()?;
    build_rpm()?;

    Ok(())
}
