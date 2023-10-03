use tnn::call;
use tnn::extension::Call;
use super::NAME;

pub struct EncryptArgs {
    pub master_password: String,
    pub value: String,
}

pub const ENCRYPT: Call<EncryptArgs, String> = call!(EncryptArgs, String, "Encrypt", NAME);