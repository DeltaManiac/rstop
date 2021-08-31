use human_panic::setup_panic;
use restop_cli::print_error;
use restop_cli::print_info;
use restop_lib;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: "DeltaManiac".into(),
        homepage: "DeltaManiac".into(),
    });
    env_logger::init();

    let opts = restop_cli::init().get_matches();
    match opts.subcommand() {
        Some(("info", info_matches)) => {
            match restop_lib::postman::spec(
                Path::new(info_matches.value_of("collection").unwrap()).to_path_buf(),
            ) {
                Ok(info) => print_info(info),
                Err(e) => print_error(
                    e,
                    Some(info_matches.value_of("collection").unwrap().to_string()),
                ),
            };
        }
        _ => unreachable!(),
    }
    Ok(())
}
