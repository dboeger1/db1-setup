use crate::{
    error::Error,
    platform::PLATFORM,
    values::DIR_PROJECT_PACKAGES,
};


pub(crate) fn package() -> Result<(), Error> {
    if DIR_PROJECT_PACKAGES.exists() {
        return Err(Error {
            message: format!(
                "cannot overwrite existing directory: \"{}\"",
                DIR_PROJECT_PACKAGES.to_string_lossy(),
            ),
            source: None,
        });
    }

    let platform_data = PLATFORM.as_ref().unwrap();
    (platform_data.archive_sources)()?;
    (platform_data.package)()?;

    Ok(())
}
