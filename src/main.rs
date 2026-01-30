use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tnn::EncryptedProperties;

#[derive(Parser, Debug)]
#[command(name = "tnn", author, about = "A command line tool for encrypting and decrypting strings", version, long_about = None)]
struct App {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand, Debug)]
enum AppCommand {
    /// Encrypt a string
    Encrypt(EncryptArgs),
    /// Decrypt an encrypted string
    Decrypt(DecryptArgs),
}

#[derive(Args, Debug)]
struct EncryptArgs {
    /// Master password (base64-encoded AES key)
    pub master_password: String,

    /// The string to encrypt
    pub value: String,
}

#[derive(Args, Debug)]
struct DecryptArgs {
    /// Master password (base64-encoded AES key)
    pub master_password: String,

    /// The encrypted string to decrypt
    pub value: String,
}

fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        AppCommand::Encrypt(args) => {
            let encrypted_properties = EncryptedProperties::new(&args.master_password)?;
            let encrypted = encrypted_properties.encrypt(&args.value)?;
            println!("{}", encrypted);
        }
        AppCommand::Decrypt(args) => {
            let encrypted_properties = EncryptedProperties::new(&args.master_password)?;
            let decrypted = encrypted_properties.decrypt(&args.value)?;
            println!("{}", decrypted);
        }
    }

    Ok(())
}
