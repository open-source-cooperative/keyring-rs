use clap::{Args, Parser};
use keyring_core::{Entry, Error, Result};
use rust_native_keyring::{
    internalize, release_store, use_apple_native_store, use_dbus_secret_service_store,
    use_linux_keyutils_store, use_sample_store, use_windows_native_store,
};
use std::collections::HashMap;

fn main() {
    let args: Cli = Cli::parse();
    set_store(&args);
    let entry = match args.entry_for() {
        Ok(entry) => entry,
        Err(err) => {
            let description = args.description();
            eprintln!("Couldn't create entry for '{description}': {err:?}");
            std::process::exit(1)
        }
    };
    match &args.command {
        Command::Set { .. } => {
            let value = args.get_password_and_attributes();
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
                    let attrs: HashMap<&str, &str> = attributes
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    match entry.update_attributes(&attrs) {
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
    let modifiers = Some(parse_attributes(rest.to_string()));
    let mods = internalize(&modifiers);
    match name {
        "sample" => use_sample_store(&mods),
        "apple" => use_apple_native_store(&mods),
        "windows" => use_windows_native_store(&mods),
        "keyutils" => use_linux_keyutils_store(&mods),
        "secret-service" => use_dbus_secret_service_store(&mods),
        _ => {
            eprintln!("Unknown credential-store module: {}", args.module);
            eprintln!("Available modules are: sample, apple, windows, keyutils, secret-service");
            std::process::exit(1);
        }
    }
    .unwrap_or_else(|err| {
        eprintln!(
            "Couldn't initialize {} credential store: {:?}",
            args.module, err
        );
        std::process::exit(1);
    });
    if let Some(mods) = modifiers
        && !mods.is_empty()
    {
        eprintln!("Using the {name} credential store with the following attributes:");
        eprint_attributes(&mods)
    } else {
        eprintln!("Using the {name} credential store");
    }
}

#[derive(Debug, Parser)]
#[clap(author = "github.com/open-source-cooperative/keyring-rs")]
/// Keyring CLI: A command-line interface to platform secure storage
pub struct Cli {
    #[clap(
        short,
        long,
        value_parser,
        default_value = "sample:backing-file=/tmp/keyring-sample-data.ron"
    )]
    /// The credential store module to use.
    pub module: String,

    #[clap(short, long, value_parser, default_value = "keyring-cli")]
    /// The service for the entry.
    pub service: String,

    #[clap(short, long, value_parser, default_value = "keyring-user")]
    /// The user for the entry.
    pub user: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Set the password or update the attributes in the secure store
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
    #[clap(short, long, action, help = "The input is a password")]
    password: bool,

    #[clap(short, long, action, help = "The input is a base64-encoded secret")]
    secret: bool,

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
                eprintln!("No credential found for '{description}'");
            }
            Error::Ambiguous(creds) => {
                eprintln!("More than one credential found for '{description}':");
                for (i, cred) in creds.iter().enumerate() {
                    eprintln!("{: >4}: {cred:?}", i + 1);
                }
            }
            err => match self.command {
                Command::Set { .. } => {
                    eprintln!("Couldn't set credential data for '{description}': {err:?}");
                }
                Command::Password => {
                    eprintln!("Couldn't get password for '{description}': {err:?}");
                }
                Command::Secret => {
                    eprintln!("Couldn't get secret for '{description}': {err:?}");
                }
                Command::Attributes => {
                    eprintln!("Couldn't get attributes for '{description}': {err:?}");
                }
                Command::Credential => {
                    eprintln!("Couldn't get credential for '{description}': {err:?}");
                }
                Command::Delete => {
                    eprintln!("Couldn't delete credential for '{description}': {err:?}");
                }
            },
        }
        std::process::exit(1)
    }

    fn success_message_for(&self, value: &Value) {
        let description = self.description();
        match self.command {
            Command::Set { .. } => match value {
                Value::Secret(secret) => {
                    let secret = secret_string(secret);
                    eprintln!("Set secret for '{description}' to decode of '{secret}'");
                }
                Value::Password(password) => {
                    eprintln!("Set password for '{description}' to '{password}'");
                }
                Value::Attributes(attributes) => {
                    eprintln!("The following attributes for '{description}' were sent for update:");
                    eprint_attributes(attributes);
                }
                _ => panic!("Can't set without a value"),
            },
            Command::Password => {
                match value {
                    Value::Password(password) => {
                        eprintln!("Password for '{description}' is '{password}'");
                    }
                    _ => panic!("Wrong value type for command"),
                };
            }
            Command::Secret => match value {
                Value::Secret(secret) => {
                    let encoded = secret_string(secret);
                    eprintln!("Secret for '{description}' encodes as {encoded}");
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Attributes => match value {
                Value::Attributes(attributes) => {
                    if attributes.is_empty() {
                        eprintln!("No attributes found for '{description}'");
                    } else {
                        eprintln!("Attributes for '{description}' are:");
                        eprint_attributes(attributes);
                    }
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Credential => match value {
                Value::Credential(credential) => {
                    eprintln!("Credential for '{description}' is: {credential:?}");
                }
                _ => panic!("Wrong value type for command"),
            },
            Command::Delete => {
                eprintln!("Successfully deleted credential for '{description}'");
            }
        }
    }

    fn get_password_and_attributes(&self) -> Value {
        if let Command::Set { what, input } = &self.command {
            if what.password {
                Value::Password(read_password(input))
            } else if what.secret {
                Value::Secret(decode_secret(input))
            } else {
                Value::Attributes(read_and_parse_attributes(input))
            }
        } else {
            panic!("Can't happen: asking for password and attributes on non-set command")
        }
    }
}

fn secret_string(secret: &[u8]) -> String {
    use base64::prelude::*;

    BASE64_STANDARD.encode(secret)
}

fn eprint_attributes(attributes: &HashMap<String, String>) {
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
            eprintln!("Sorry, the provided secret data is not base64-encoded: {err:?}");
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
        eprintln!("You must specify at least one key=value attribute pair to set");
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
            eprintln!("Sorry, this part of the attributes string is not a key=val pair: {s}");
            std::process::exit(1);
        }
        attributes.insert(parts[0].to_string(), parts[1].to_string());
    }
    attributes
}
