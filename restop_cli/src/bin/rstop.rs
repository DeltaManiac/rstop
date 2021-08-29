use human_panic::setup_panic;
use log::{debug, error, info, log_enabled, Level};
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
            let c = restop_lib::postman::spec(
                Path::new(info_matches.value_of("collection").unwrap()).to_path_buf(),
            )?;
            print_info(c);
        }
        _ => unreachable!(),
    }
    Ok(())
}
