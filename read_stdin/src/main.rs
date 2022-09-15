use clap::{arg, command, crate_version, AppSettings, Command};
use const_format::formatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::io;
use std::io::BufRead;
use std::net::Ipv4Addr;

fn main() {
    let version_string: &str = formatcp!("{}", crate_version!());

    /*let mut buffer = String::new();
    let _bytes_read = io::stdin().lock().read_line(&mut buffer).unwrap();
    let hostname = buffer.lines().next().unwrap();*/

    let valid_host = valid_length;
    let hostname = read_interactive_input("Enter Host: ", &valid_host);
    println!("hostname: {}", hostname);

    /*    // let version_string: &str = formatcp!("{} ({})", crate_version!(), option_env!("VERGEN_GIT_SHA"));
    let arg_matches = command!()
        .arg_required_else_help(true)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .propagate_version(false)
        .subcommands(vec![
            Command::new("db")
                .about("Database CRUD commands")
                .arg_required_else_help(true)
                .args(&[
                    arg!(-a --add     "Adds record to a DB"),
                    arg!(-d --delete  "Deletes a record from DB"),
                ]),
            Command::new("config")
                .about("Config command")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommands(vec![Command::new("resource")
                    .about("resource request")
                    .arg_required_else_help(true)
                    .args(&[
                        arg!(--core <CORES> "number of cores"),
                        arg!(--memory <MEMORY> "memory in GB"),
                    ])]),
        ])
        .version(version_string)
        .get_matches();

    match arg_matches.subcommand() {
        Some(("db", db_arg_matches)) => {
            if db_arg_matches.contains_id("add") {
                println!("Add")
            } else if db_arg_matches.contains_id("delete") {
                println!("Delete")
            } else {
                println!("Not a valid")
            }
        }
        Some(("config", config_arg_matches)) => {
            match config_arg_matches.subcommand() {
                Some(("resource", resource_arg_matches)) => {
                    // println!("{}", resource_arg_matches.try_get_one::<u16>("core").unwrap_or_else(Some(Default::default())).unwrap());
                    // println!("{}", resource_arg_matches.try_get_one::<String>("memory").unwrap_or_else(Some("10 GB")).unwrap());
                    println!(
                        "{}",
                        resource_arg_matches.get_one::<String>("core").unwrap()
                    );
                    println!(
                        "{}",
                        resource_arg_matches.get_one::<String>("memory").unwrap()
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }; */
}

fn read_interactive_input<'a>(
    cli_prompt: &str,
    validation_func: &'a dyn Fn(&str) -> bool,
) -> String {
    loop {
        let mut buffer = String::new();
        match io::stdin().lock().read_line(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let input = buffer.lines().next().unwrap();
                    if validation_func(input.clone()) {
                        break input.to_string();
                    }
                } /*else {}*/
            }
            Err(_) => {}
        }
    }
}

fn valid_length(input: &str) -> bool {
    if input.len() > 0 {
        return true;
    }

    false
}
/* /// Returns true if input is a valid hostname as per the definition
/// at https://man7.org/linux/man-pages/man7/hostname.7.html, otherwise false
fn valid_hostname(input: &str) -> bool {
    if input.len() == 0 || input.len() > 253 {
        return false;
    }
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(([a-zA-Z0-9]{1,63}|[a-zA-Z0-9][a-zA-Z0-9\-]{0,62})\.)*([a-zA-Z0-9]{1,63}|[a-zA-Z0-9][a-zA-Z0-9\-]{0,62})$").unwrap();
    }
    RE.is_match(input)
}

/// Returns true if input is a valid IPv4 address, otherwise false
fn valid_ipv4_address(input: &str) -> bool {
    match input.parse::<Ipv4Addr>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn valid_host(input: &str) -> bool {
    valid_ipv4_address(input) || valid_hostname(input)
} */
