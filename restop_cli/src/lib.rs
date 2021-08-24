use clap::crate_authors;
use clap::crate_license;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::ValueHint;

pub fn init() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate::crate_authors!())
        .license(crate::crate_license!())
        .subcommand(
            App::new("run")
                .about("Start a postman collection run")
                .arg(
                    Arg::new("collection")
                        .about("Path to postman collection")
                        .value_name("File")
                        .forbid_empty_values(true)
                        .takes_value(true)
                        .value_hint(ValueHint::FilePath),
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .setting(AppSettings::ColoredHelp)
                .setting(AppSettings::Built),
        )
        .subcommand(
            App::new("info")
                .about("Print information about the postman collection")
                .arg(
                    Arg::new("collection")
                        .about("Path to postman collection")
                        .value_name("File")
                        .forbid_empty_values(true)
                        .takes_value(true)
                        .value_hint(ValueHint::FilePath),
                )
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .setting(AppSettings::ColoredHelp)
                .setting(AppSettings::Built),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::Built)
}
