use tnn::call;
use tnn::extension::Call;
use super::NAME;

pub struct DecryptArgs {
    pub master_password: String,
    pub value: String,
}

pub const DECRYPT: Call<DecryptArgs, String> = call!(DecryptArgs, String, "Decrypt", NAME);