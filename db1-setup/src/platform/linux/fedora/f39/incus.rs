use crate::error::Error;


pub(crate) fn install() -> Result<(), Error> {
    println!("<< F39 INCUS INSTALL >>");

    Ok(())
}

pub(crate) fn verify() -> Result<(), Error> {
    println!("<< F39 INCUS VERIFY >>");

    Ok(())
}

//    install(["dnf-command(copr)"])?;
//    copr_enable("ganto/lxc4")?;
//incus
