use ansi_term::Color::Yellow;
use std::process::Command;
use structopt::StructOpt;

const PACKAGE_MANAGER: &str = "pacman";
const BASE_QUERY_FLAGS: &str = "-Qq";
const INFO_QUERY_FLAGS: &str = "-Qi";
const OPTIONAL_DEPS_STR: &str = "Optional Deps";
const OPTIONAL_DEPS_SEPERATOR_STR: &str = "   : ";
const REQUIRED_BY_STR: &str = "Required By";
const INSTALLED_STR: &str = "[installed]";
const NONE_STR: &str = "None";

/// Find optional dependencies for an Arch Linux installation
#[derive(StructOpt)]
struct Cli {
    /// Only list dependencies for explicitly installed packages
    #[structopt(short, long)]
    explicit: bool,
    /// Include dependencies that are already installed
    #[structopt(short, long)]
    installed: bool,
}

fn main() {
    let config = Cli::from_args();

    run(&config);
}

fn run(config: &Cli) {
    // Create the flags required to get the packages from pacman, including the cli options
    let mut flags = String::from(BASE_QUERY_FLAGS);
    flags.push_str(match config.explicit {
        true => "e",
        false => "",
    });

    // Get a list of all the packages
    let packages = get_packages(flags.as_str());

    // Get the optional dependencies for each package, filter, and print
    for package in packages.lines() {
        // Get the full package details
        let package_details = get_package_details(package);

        // Find the range from 'Optional Deps' to 'Required By' to get all optional dependencies
        // Ex: Firefox
        // ...
        // Depends On      : gtk3  libxt  mime-types  dbus-glib  ffmpeg  nss  ttf-font  libpulse
        // Optional Deps   : networkmanager: Location detection via available WiFi networks [installed]
        //                   libnotify: Notification integration [installed]
        //                   pulseaudio: Audio support [installed]
        //                   speech-dispatcher: Text-to-Speech
        //                   hunspell-en_US: Spell checking, American English
        // Required By     : None
        // ...
        let opt_depends_range = (
            package_details.rfind(OPTIONAL_DEPS_STR).unwrap(),
            package_details.find(REQUIRED_BY_STR).unwrap(),
        );

        let opt_depends = String::from(&package_details[opt_depends_range.0..opt_depends_range.1]);
        let opt_depends = opt_depends.replace(
            &format!("{}{}", OPTIONAL_DEPS_STR, OPTIONAL_DEPS_SEPERATOR_STR),
            "",
        );

        // Ignore packages with no optional dependencies
        if opt_depends.contains(NONE_STR) {
            continue;
        }

        // Filter and format the dependencies based on the cli flags
        let opt_depends = filter_optional_dependencies(opt_depends, config.installed);

        // Print the optional dependencies
        print(opt_depends, package);
    }
}

fn get_packages(flags: &str) -> String {
    // Run the command required to get all installed packages
    String::from_utf8(
        Command::new(PACKAGE_MANAGER)
            .arg(flags)
            .output()
            .expect("Failed to retrieve installed packages") /* TODO: Better error handling */
            .stdout,
    )
    .unwrap()
}

fn get_package_details(package: &str) -> String {
    // Run the command required to get the details for a package
    String::from_utf8(
        Command::new(PACKAGE_MANAGER)
            .arg(INFO_QUERY_FLAGS)
            .arg(package)
            .output()
            .expect(format!("Failed to retrieve package details for `{}`", package).as_str()) /* TODO: Better error handling */
            .stdout,
    )
    .unwrap()
}

fn filter_optional_dependencies(opt_depends: String, installed: bool) -> Vec<String> {
    let mut depends: Vec<String> = Vec::new();

    // Trim each line and push it to the depends vec depending on the cli flags
    for line in opt_depends.lines() {
        let line = String::from(line);
        let line = String::from(line.trim());

        if line.contains(INSTALLED_STR) && installed {
            depends.push(line);
        } else if !line.contains(INSTALLED_STR) {
            depends.push(line);
        }
    }

    depends
}

fn print(opt_depends: Vec<String>, package: &str) {
    // Print package names followed by their optional dependencies
    if opt_depends.len() != 0 {
        println!("{}:", Yellow.bold().paint(package));
        for opt_dependency in opt_depends {
            println!("    {}", opt_dependency);
        }
    }
}
