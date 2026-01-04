//! An exceedingly simple CLI for the Rust keyring crate.
//!
//! This is more sample code than anything else because each command requires
//! a separate invocation, including connecting to and disconnecting from a store.
//! Invoke this command with no arguments to see usage information.
use clap::{Args, Parser};
use std::collections::HashMap;

use keyring::{
    internalize, release_store, store_info, use_named_store, use_named_store_with_modifiers,
};
use keyring_core::{Entry, Error, Result};

fn main() {
    let args: Cli = Cli::parse();
    set_store(&args);
    let entry = match args.entry_for() {
        Ok(entry) => entry,
        Err(err) => {
            let description = args.description();
            println!("Couldn't create entry for '{description}': {err:?}");
            std::process::exit(1)
        }
    };
    match &args.command {
        Command::Info => {
            println!("Store info: {}", store_info());
            println!("Entry info: {entry:?}");
        }
        Command::Set { .. } => {
            let value = args.read_value_to_set();
            match &value {
                Value::Secret(secret) => match entry.set_secret(secret) {
                    Ok(()) => args.success_message_for(&value),
                    Err(err) => args.error_message_for(err),
                },
                Value::Password(password) => match entry.set_password(password) {
                    Ok(()) => args.success_message_for(&value),
                    Err(err) => args.error_message_for(err),
                },
                Value::Attributes(attributes) => {
                    match entry.update_attributes(&internalize(Some(attributes))) {
                        Ok(()) => args.success_message_for(&value),
                        Err(err) => args.error_message_for(err),
                    }
                }
                _ => panic!("Can't set without a value"),
            }
        }
        Command::Password => match entry.get_password() {
            Ok(password) => {
                args.success_message_for(&Value::Password(password));
            }
            Err(err) => args.error_message_for(err),
        },
        Command::Secret => match entry.get_secret() {
            Ok(secret) => {
                args.success_message_for(&Value::Secret(secret));
            }
            Err(err) => args.error_message_for(err),
        },
        Command::Attributes => match entry.get_attributes() {
            Ok(attributes) => {
                args.success_message_for(&Value::Attributes(attributes));
            }
            Err(err) => args.error_message_for(err),
        },
        Command::Credential => match entry.get_credential() {
            Ok(credential) => args.success_message_for(&Value::Credential(credential)),
            Err(err) => args.error_message_for(err),
        },
        Command::Delete => match entry.delete_credential() {
            Ok(()) => args.success_message_for(&Value::None),
            Err(err) => args.error_message_for(err),
        },
    }
    release_store()
}

fn set_store(args: &Cli) {
    let (name, rest) = args
        .module
        .split_once(':')
        .unwrap_or((args.module.as_str(), ""));
    if rest.is_empty() {
        use_named_store(name).unwrap_or_else(|err| {
            println!("{err}");
            std::process::exit(1);
        });
        println!("Using the {name} credential store");
    } else {
        let modifiers = Some(parse_attributes(rest.to_string()));
        let mods = internalize(modifiers.as_ref());
        use_named_store_with_modifiers(name, &mods).unwrap_or_else(|err| {
            println!("{err}");
            std::process::exit(1);
        });
        println!("Using the {name} credential store with the following attributes:");
        print_attributes(modifiers.as_ref().unwrap());
    }
}

#[derive(Debug, Parser)]
#[clap(author = "github.com/open-source-cooperative/keyring-rs")]
/// Keyring CLI: A command-line interface to platform secure storage
pub struct Cli {
    #[clap(global = true, short, long, value_parser, default_value = "sample")]
    /// The credential store module to use.
    pub module: String,

    #[clap(
        global = true,
        short,
        long,
        value_parser,
        default_value = "keyring-cli"
    )]
    /// The service for the entry.
    pub service: String,

    #[clap(
        global = true,
        short,
        long,
        value_parser,
        default_value = "keyring-user"
    )]
    /// The user for the entry.
    pub user: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Show info about the store and entry in use.
    Info,
    /// Set the password/secret or update the attributes in the secure store
    Set {
        #[command(flatten)]
        what: What,

        #[clap(value_parser)]
        /// The input to parse. If not specified, it will be
        /// read interactively from the terminal. Password/secret
        /// input will not be echoed.
        input: Option<String>,
    },
    /// Retrieve the (string) password from the secure store
    /// and write it to the standard output.
    Password,
    /// Retrieve the (binary) secret from the secure store
    /// and write it in base64 encoding to the standard output.
    Secret,
    /// Retrieve attributes available in the secure store.
    Attributes,
    /// Retrieve the credential from the secure store.
    Credential,
    /// Delete the credential from the secure store.
    Delete,
}

#[derive(Debug, Args)]
#[group(multiple = false, required = true)]
pub struct What {
    #[clap(short, long, action, help = "The input is a utf8-encoded password")]
    password: bool,

    #[clap(short, long, action, help = "The input is a base64-encoded blob")]
    blob: bool,

    #[clap(
        short,
        long,
        action,
        help = "The input is comma-separated, key=val attribute pairs"
    )]
    attributes: bool,
}

enum Value {
    Secret(Vec<u8>),
    Password(String),
    Attributes(HashMap<String, String>),
    Credential(Entry),
    None,
}

impl Cli {
    fn description(&self) -> String {
        format!("{}@{}", &self.user, &self.service)
    }

    fn entry_for(&self) -> Result<Entry> {
        Entry::new(&self.service, &self.user)
    }

    fn error_message_for(&self, err: Error) {
        let description = self.description();
        match err {
            Error::NoEntry => {
                println!("No credential found for '{description}'");
            }
            Error::Ambiguous(creds) => {
                println!("More than one credential found for '{description}':");
                for (i, cred) in creds.iter().enumerate() {
                    println!("{: >4}: {cred:?}", i + 1);
                }
            }
            err => match self.command {
                Command::Info => panic!("Can't happen: info command should never fail"),
                Command::Set { .. } => {
                    println!("Couldn't set credential data for '{description}': {err:?}");
                }
                Command::Password => {
                    println!("Couldn't get password for '{description}': {err:?}");
                }
                Command::Secret => {
                    println!("Couldn't get secret for '{description}': {err:?}");
                }
                Command::Attributes => {
                    println!("Couldn't get attributes for '{description}': {err:?}");
                }
                Command::Credential => {
                    println!("Couldn't get credential for '{description}': {err:?}");
                }
                Command::Delete => {
                    println!("Couldn't delete credential for '{description}': {err:?}");
                }
            },
        }
        std::process::exit(1)
    }

    fn success_message_for(&self, value: &Value) {
        let description = self.description();
        match self.command {
            Command::Info => panic!("Can't happen: info command should not invoke success message"),
            Command::Set { .. } => match value {
                Value::Secret(secret) => {
                    let secret = secret_string(secret);
                    println!("Set secret for '{description}' to decode of '{secret}'");
                }
                Value::Password(password) => {
                    println!("Set password for '{description}' to '{password}'");
                }
                Value::Attributes(attributes) => {
                    println!("The following attributes for '{description}' were sent for update:");
                    print_attributes(attributes);
                }
                _ => panic!("Can't set without a value"),
            },
            Command::Password => {
                match value {
                    Value::Password(password) => {
                        println!("Password for '{description}' is '{password}'");
                    }
                    _ => panic!("Wrong value type for command"),
                };
            }
            Command::Secret => match value {
                Value::Secret(secret) => {
                    let encoded = secret_string(secret);
                    println!("Secret for '{description}' encodes as {encoded}");
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Attributes => match value {
                Value::Attributes(attributes) => {
                    if attributes.is_empty() {
                        println!("No attributes found for '{description}'");
                    } else {
                        println!("Attributes for '{description}' are:");
                        print_attributes(attributes);
                    }
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Credential => match value {
                Value::Credential(credential) => {
                    println!("Credential for '{description}' is: {credential:?}");
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Delete => {
                println!("Successfully deleted credential for '{description}'");
            }
        }
    }

    fn read_value_to_set(&self) -> Value {
        if let Command::Set { what, input } = &self.command {
            if what.password {
                Value::Password(read_password(input))
            } else if what.blob {
                Value::Secret(decode_secret(input))
            } else {
                Value::Attributes(read_and_parse_attributes(input))
            }
        } else {
            panic!("Can't happen: only set command takes a value input")
        }
    }
}

fn secret_string(secret: &[u8]) -> String {
    use base64::prelude::*;

    BASE64_STANDARD.encode(secret)
}

fn print_attributes(attributes: &HashMap<String, String>) {
    for (key, value) in attributes {
        println!("    {key}: {value}");
    }
}

fn decode_secret(input: &Option<String>) -> Vec<u8> {
    use base64::prelude::*;

    let encoded = if let Some(input) = input {
        input.clone()
    } else {
        rpassword::prompt_password("Base64 encoding: ").unwrap_or_else(|_| String::new())
    };
    if encoded.is_empty() {
        return Vec::new();
    }
    match BASE64_STANDARD.decode(encoded) {
        Ok(secret) => secret,
        Err(err) => {
            println!("Sorry, the provided secret data is not base64-encoded: {err:?}");
            std::process::exit(1);
        }
    }
}

fn read_password(input: &Option<String>) -> String {
    if let Some(input) = input {
        input.clone()
    } else {
        rpassword::prompt_password("Password: ").unwrap_or_else(|_| String::new())
    }
}

fn read_and_parse_attributes(input: &Option<String>) -> HashMap<String, String> {
    let input = if let Some(input) = input {
        input.clone()
    } else {
        rprompt::prompt_reply("Attributes: ").unwrap_or_else(|_| String::new())
    };
    if input.is_empty() {
        println!("You must specify at least one key=value attribute pair to set");
        std::process::exit(1);
    }
    parse_attributes(input)
}

fn parse_attributes(input: String) -> HashMap<String, String> {
    let mut attributes = HashMap::new();
    if input.is_empty() {
        return attributes;
    }
    let parts = input.split(',');
    for s in parts.into_iter() {
        let parts: Vec<&str> = s.split("=").collect();
        if parts.len() != 2 || parts[0].is_empty() {
            println!("Sorry, this part of the attributes string is not a key=val pair: {s}");
            std::process::exit(1);
        }
        attributes.insert(parts[0].to_string(), parts[1].to_string());
    }
    attributes
}
