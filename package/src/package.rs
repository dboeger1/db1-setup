use crate::{
    error::Error,
    platform::Platform,
    values::DIR_PROJECT_PACKAGES,
};


pub(crate) fn package(platform: &Platform) -> Result<(), Error> {
    if DIR_PROJECT_PACKAGES.exists() {
        return Err(Error {
            message: format!(
                "Cannot overwrite directory: {}",
                DIR_PROJECT_PACKAGES.to_string_lossy(),
            ),
            source: None,
        });
    }

    (platform.archive_sources)()?;
    (platform.package)()?;

    Ok(())
}
