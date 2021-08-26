use std::path::Path;
use restop_lib;
fn main() {

    let opts = restop_cli::init().get_matches();
    match opts.subcommand(){

        Some(("info",info_matches)) => {
            let c = restop_lib::postman::spec(Path::new(info_matches.value_of("collection").unwrap()).to_path_buf());
            dbg!(&c);
        },
        _ => unreachable!()
    }
}


// use clap::{Arg, App};

// fn main() {
//     let matches = App::new("My Super Program")
//         .version("1.0")
//         .author("Kevin K. <kbknapp@gmail.com>")
//         .about("Does awesome things")
//         .arg(Arg::new("config")
//             .short('c')
//             .long("config")
//             .value_name("FILE")
//             .about("Sets a custom config file")
//             .takes_value(true))
//         .arg(Arg::new("INPUT")
//             .about("Sets the input file to use")
//             .required(true)
//             .index(1))
//         .arg(Arg::new("v")
//             .short('v')
//             .multiple_occurrences(true)
//             .takes_value(true)
//             .about("Sets the level of verbosity"))
//         .subcommand(App::new("test")
//             .about("controls testing features")
//             .version("1.3")
//             .author("Someone E. <someone_else@other.com>")
//             .arg(Arg::new("debug")
//                 .short('d')
//                 .about("print debug information verbosely")))
//         .get_matches();

//     // You can check the value provided by positional arguments, or option arguments
//     if let Some(i) = matches.value_of("INPUT") {
//         println!("Value for input: {}", i);
//     }

//     if let Some(c) = matches.value_of("config") {
//         println!("Value for config: {}", c);
//     }

//     // You can see how many times a particular flag or argument occurred
//     // Note, only flags can have multiple occurrences
//     match matches.occurrences_of("v") {
//         0 => println!("Verbose mode is off"),
//         1 => println!("Verbose mode is kind of on"),
//         2 => println!("Verbose mode is on"),
//         _ => println!("Don't be crazy"),
//     }

//     // You can check for the existence of subcommands, and if found use their
//     // matches just as you would the top level app
//     if let Some(ref matches) = matches.subcommand_matches("test") {
//         // "$ myapp test" was run
//         if matches.is_present("debug") {
//             // "$ myapp test -d" was run
//             println!("Printing debug info...");
//         } else {
//             println!("Printing normally...");
//         }
//     }

//     // Continued program logic goes here...
// }

// use clap::{App, AppSettings, Arg};

// fn main() {
//     let matches = App::new("git")
//         .about("A fictional versioning CLI")
//         .version("1.0")
//         .author("Me")
//         .subcommand(
//             App::new("clone")
//                 .about("clones repos")
//                 .license("MIT OR Apache-2.0")
//                 .arg(Arg::new("repo").about("The repo to clone").required(true)),
//         )
//         .subcommand(
//             App::new("push")
//                 .about("pushes things")
//                 .setting(AppSettings::SubcommandRequiredElseHelp)
//                 .subcommand(
//                     App::new("remote") // Subcommands can have their own subcommands,
//                         // which in turn have their own subcommands
//                         .about("pushes remote things")
//                         .arg(
//                             Arg::new("repo")
//                                 .required(true)
//                                 .about("The remote repo to push things to"),
//                         ),
//                 )
//                 .subcommand(App::new("local").about("pushes local things")),
//         )
//         .subcommand(
//             App::new("add")
//                 .about("adds things")
//                 .author("Someone Else") // Subcommands can list different authors
//                 .version("v2.0 (I'm versioned differently") // or different version from their parents
//                 .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
//                 .arg(
//                     Arg::new("stuff")
//                         .long("stuff")
//                         .about("Stuff to add")
//                         .takes_value(true)
//                         .multiple_values(true),
//                 ),
//         )
//         .get_matches();

//     // At this point, the matches we have point to git. Keep this in mind...

//     // You can check if one of git's subcommands was used
//     if matches.is_present("clone") {
//         println!("'git clone' was run.");
//     }

//     // You can see which subcommand was used
//     if let Some(subcommand) = matches.subcommand_name() {
//         println!("'git {}' was used", subcommand);

//         // It's important to note, this *only* check's git's DIRECT children, **NOT** it's
//         // grandchildren, great grandchildren, etc.
//         //
//         // i.e. if the command `git push remove --stuff foo` was run, the above will only print out,
//         // `git push` was used. We'd need to get push's matches to see further into the tree
//     }

//     // An alternative to checking the name is matching on known names. Again notice that only the
//     // direct children are matched here.
//     match matches.subcommand_name() {
//         Some("clone") => println!("'git clone' was used"),
//         Some("push") => println!("'git push' was used"),
//         Some("add") => println!("'git add' was used"),
//         None => println!("No subcommand was used"),
//         _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
//     }

//     // You could get the independent subcommand matches, although this is less common
//     if let Some(clone_matches) = matches.subcommand_matches("clone") {
//         // Now we have a reference to clone's matches
//         println!("Cloning repo: {}", clone_matches.value_of("repo").unwrap());
//     }

//     // The most common way to handle subcommands is via a combined approach using
//     // `ArgMatches::subcommand` which returns a tuple of both the name and matches
//     match matches.subcommand() {
//         Some(("clone", clone_matches)) => {
//             // Now we have a reference to clone's matches
//             println!("Cloning {}", clone_matches.value_of("repo").unwrap());
//         }
//         Some(("push", push_matches)) => {
//             // Now we have a reference to push's matches
//             match push_matches.subcommand() {
//                 Some(("remote", remote_matches)) => {
//                     // Now we have a reference to remote's matches
//                     println!("Pushing to {}", remote_matches.value_of("repo").unwrap());
//                 }
//                 Some(("local", _)) => {
//                     println!("'git push local' was used");
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         Some(("add", add_matches)) => {
//             // Now we have a reference to add's matches
//             println!(
//                 "Adding {}",
//                 add_matches
//                     .values_of("stuff")
//                     .unwrap()
//                     .collect::<Vec<_>>()
//                     .join(", ")
//             );
//         }
//         None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
//         _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
//     }

//     // Continued program logic goes here...
// }