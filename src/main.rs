/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                +--------------------------------------------+              *
 *                |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |              *
 *                |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |              *
 *                |  ███████║██║   ██║██████╔╝███████║█████╗   |              *
 *                |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |              *
 *                |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |              *
 *                |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |              *
 *                +--------------------------------------------+              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * -------------------------------------------------------------------------- *
 *                                                                            *
 *   Licensed under the Apache License, Version 2.0 (the "License");          *
 *   you may not use this file except in compliance with the License.         *
 *   You may obtain a copy of the License at                                  *
 *                                                                            *
 *       http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                            *
 *   Unless required by applicable law or agreed to in writing, software      *
 *   distributed under the License is distributed on an "AS IS" BASIS,        *
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *   See the License for the specific language governing permissions and      *
 *   limitations under the License.                                           *
 *                                                                            *
\* -------------------------------------------------------------------------- */

mod config;
mod pki;

extern crate core;

use crate::config::Settings;
use crate::pki::generate_keypair;
use clap::*;
use std::path::PathBuf;
use syslog::*;

const EXIT_OKAY: i32 = 0;
const EXIT_ERROR: i32 = 1;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn on verbose output
    #[clap(short, long)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// All things related PKI (certificates, ssh keys, ...)
    Pki(Pki),
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct Pki {
    #[clap(subcommand)]
    command: PkiCommands,
}

#[derive(Debug, Subcommand)]
enum PkiCommands {
    /// All things SSH (key generation, fingerprinting, ...)
    Ssh(Ssh),
}

#[derive(Debug, Args)]
struct Ssh {
    #[clap(subcommand)]
    command: SshCommands,
}

#[derive(Debug, Subcommand)]
enum SshCommands {
    /// Generate a new ED25559 SSH keypair
    Generate {},
    /// Print the fingerprint of a given SSH key
    Print {},
}

fn run() -> i32 {
    let settings = Settings::new().unwrap();
    println!("settings: {:?}", settings);

    let args: Cli = Cli::parse();
    let name = "auraectl";

    // The logger will log to stdout and the syslog by default.
    // We hold the opinion that the program is either "verbose"
    // or it's not.
    //
    // Normal mode: Info, Warn, Error
    // Verbose mode: Debug, Trace, Info, Warn, Error
    let logger_level = if args.verbose {
        log::Level::Trace
    } else {
        log::Level::Info
    };

    // Syslog formatter
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: name.into(),
        pid: 0,
    };

    // Initialize the logger
    let logger_simple =
        simplelog::SimpleLogger::new(logger_level.to_level_filter(), simplelog::Config::default());
    let logger_syslog = syslog::unix(formatter).unwrap();
    let _ = match multi_log::MultiLogger::init(
        vec![logger_simple, Box::new(BasicLogger::new(logger_syslog))],
        logger_level,
    ) {
        Ok(_) => {}
        Err(e) => panic!("unable to connect to syslog: {:?}", e),
    };

    let res = match args.command {
        Commands::Pki(pki) => match pki.command {
            PkiCommands::Ssh(ssh) => match ssh.command {
                SshCommands::Generate {} => generate_keypair(&settings),
                SshCommands::Print {} => Ok(()),
            },
        },
    };

    match res {
        Err(e) => {
            println!("error: {}", e);
            EXIT_ERROR
        }
        Ok(()) => EXIT_OKAY,
    }
}

fn main() {
    let exit_code = run();
    std::process::exit(exit_code);
}
