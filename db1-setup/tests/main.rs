#[test]
fn test_args() {
    let _args = db1_setup::Args {
        subcommand: db1_setup::Subcommand::Cpp(
            db1_setup::cpp::Args {
                subcommand: db1_setup::cpp::Subcommand::Install,
            },
        ),
    };
}
