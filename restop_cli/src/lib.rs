use std::path::PathBuf;

use clap::ValueHint;
use clap::{crate_authors, crate_name, crate_version};
use clap::{App, AppSettings};
use clap::{Arg, ArgSettings};
use colored::*;
use restop_lib::postman::PostmanInfo;
use restop_lib::postman::RestopLibError;
use restop_lib::postman::SpecVersion;

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

macro_rules! printKV {
    ($k:expr,$v:expr) => {{
        println!("{} : {}", $k.red().bold(), $v.blue());
    }};
}

pub fn print_info(info: PostmanInfo) {
    println!("{}", "Information".yellow().bold());
    let ver = match info.spec {
        SpecVersion::V2_0_0 => "v2.0.0".to_owned(),
        SpecVersion::V2_1_0 => "v2.1.0".to_owned(),
    };
    printKV!("Name", info.information.name);
    printKV!(
        "Postman Id",
        info.information.postman_id.unwrap_or(String::from("None"))
    );
    printKV!(
        "Description",
        info.information.description.unwrap_or(String::from("None"))
    );
    printKV!("Specification Version", ver);
    printKV!("Schema", info.information.schema);
    printKV!(
        "Version",
        info.information.version.unwrap_or(String::from("None"))
    );
    printKV!("Items", info.items.to_string());
    printKV!("Events", info.events.to_string());
    printKV!("Variables", info.events.to_string());
}

pub fn print_error(error: RestopLibError, path: Option<String>) {
    match error {
        RestopLibError::PostmanCollectionError(err) => {
            eprintln!(
                "{}: {} at {} ",
                "error".red(),
                err,
                path.unwrap_or("".to_string())
            )
        }
        _ => {
            eprintln!("{}: {} ", "error".red(), error)
        }
    }
}
