//! A bytemuck cast extension trait for CredentialApi

use bytemuck::{fill_zeroes, NoUninit, Pod};

use crate::credential::CredentialApi;
use crate::Result;

// only manually impl if you store is remote as default impl does not convert endians
pub trait CredentialApiCast {
    fn set_secret_cast<T: Pod + NoUninit>(&self, secret: &T) -> Result<()>;
    fn get_secret_cast<T: Pod + NoUninit>(&self) -> Result<T>;
}

impl<Api: CredentialApi> CredentialApiCast for Api {
    fn set_secret_cast<T: NoUninit>(&self, secret: &T) -> Result<()> {
        self.set_secret(bytemuck::bytes_of(secret))
    }

    fn get_secret_cast<T: Pod>(&self) -> Result<T> {
        let mut secret = self.get_secret()?;
        let casted = *bytemuck::try_from_bytes::<T>(&secret)?;

        // zero what the user can't control
        fill_zeroes(&mut secret);
        drop(secret);

        Ok(casted)
    }
}
