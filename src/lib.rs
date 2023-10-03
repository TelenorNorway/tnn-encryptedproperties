pub mod api;

mod encrypted_properties;
mod encrypted_properties_tests;
mod encrypted_properties_error;

use anyhow::Result;
use clap::{Args, command, Parser, Subcommand};
use tnn::core::api::add_command;
use tnn::extension::{CallContext, CallOutput, Dependency, Extension, ExtensionContext};

#[no_mangle]
pub static MANIFEST: Extension = Extension {
    name: api::NAME,
    version: env!("CARGO_PKG_VERSION"),
    dependencies: &[Dependency::Required("tnn", ">=0")],
    init: &|ctx| Box::pin(async { init(ctx).await }),
};

async fn init(ctx: ExtensionContext) -> Result<()> {
    add_command(&ctx, &app).await?;
    ctx.add_call(&api::ENCRYPT, &encrypt).await?;
    ctx.add_call(&api::DECRYPT, &decrypt).await?;
    Ok(())
}

fn encrypt(ctx: CallContext<api::EncryptArgs>) -> CallOutput<String> {
    Box::pin(async move {
        let encrypted_properties =
            encrypted_properties::EncryptedProperties::new(&ctx.argument.master_password)?;
        let encrypted = encrypted_properties
            .encrypt(&ctx.argument.value)?;
        Ok(encrypted)
    })
}

fn decrypt(ctx: CallContext<api::DecryptArgs>) -> CallOutput<String> {
    Box::pin(async move {
        let encrypted_properties =
            encrypted_properties::EncryptedProperties::new(&ctx.argument.master_password)?;
        let decrypted = encrypted_properties
            .decrypt(&ctx.argument.value)?;
        Ok(decrypted)
    })
}

async fn app(args: App) -> Result<()> {
    match args.command {
        AppCommand::Decrypt(args) => {
            let encrypted_properties =
                encrypted_properties::EncryptedProperties::new(&args.master_password)?;
            let decrypted = encrypted_properties
                .decrypt(&args.encrypted_string)?;
            println!("{}", decrypted);
        }
        AppCommand::Encrypt(args) => {
            let encrypted_properties =
                encrypted_properties::EncryptedProperties::new(&args.master_password)?;
            let encrypted = encrypted_properties
                .encrypt(&args.string)?;
            println!("{}", encrypted);
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "ep", author, about, version, long_about = None)]
/// A command line tool for managing encrypted properties.
struct App {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand, Debug)]
enum AppCommand {
    /// Decrypt an encrypted string.
    Decrypt(DecryptArgs),
    /// Encrypt a string.
    Encrypt(EncryptArgs),
}

#[derive(Args, Debug)]
struct DecryptArgs {
    #[arg()]
    pub master_password: String,
    #[arg()]
    pub encrypted_string: String,
}

#[derive(Args, Debug)]
struct EncryptArgs {
    #[arg()]
    pub master_password: String,
    #[arg()]
    pub string: String,
}
