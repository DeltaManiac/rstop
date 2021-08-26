use std::path::PathBuf;

use clap::ValueHint;
use clap::{crate_authors, crate_name, crate_version};
use clap::{App, AppSettings};
use clap::{Arg, ArgSettings};

pub fn init() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            App::new("run")
                .about("Start a postman collection run")
                .arg(
                    Arg::new("collection")
                        .about("Path to postman collection")
                        .forbid_empty_values(true)
                        .takes_value(true)
                        .value_hint(ValueHint::FilePath)
                        .setting(ArgSettings::Last),
                )
                .setting(AppSettings::ColoredHelp),
        )
        .subcommand(
            App::new("info")
                .about("Print information about the postman collection")
                .arg(
                    Arg::new("collection")
                        .about("Path to postman collection")
                        .required(true)
                        .validator(check_file)
                        .value_hint(ValueHint::FilePath),
                )
                .setting(AppSettings::ColoredHelp),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
}

fn check_file(path: &str) -> Result<(), String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(String::from(
            format_args!("Could not read file at {}", path.to_str().unwrap()).to_string(),
        ));
    }
    Ok(())
}
