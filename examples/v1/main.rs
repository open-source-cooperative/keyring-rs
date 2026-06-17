use keyring::{Entry, Result};

fn main() -> Result<()> {
    let entry = Entry::new("my-service", "my-name")?;
    //noinspection SpellCheckingInspection
    entry.set_password("topS3cr3tP4$$w0rd")?;
    let password = entry.get_password()?;
    println!("My password is '{}'", password);
    entry.delete_credential()?;
    Ok(())
}
