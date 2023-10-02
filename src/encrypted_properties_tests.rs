#[cfg(test)]
mod encrypted_properties_tests {
    use crate::encrypted_properties::EncryptedProperties;

    #[test]
    fn test_decrypter_new() {
        let master_password = "wrRROrC5Ml/4ewM+HTdvgbtwS3nUiZ1KobO/Rex8xxM=";
        let decrypter = EncryptedProperties::new(master_password);
        assert!(decrypter.is_ok(), "Decrypter::new failed with valid input");
    }

    #[test]
    fn test_decrypter_new_invalid() {
        let empty_password = "";
        let decrypter = EncryptedProperties::new(empty_password);
        assert!(decrypter.is_err(), "Decrypter::new should fail with empty password");
    }

    #[test]
    fn test_decrypter_encrypt_decrypt() {
        let master_password = "wrRROrC5Ml/4ewM+HTdvgbtwS3nUiZ1KobO/Rex8xxM=";
        let decrypter = EncryptedProperties::new(master_password).unwrap();
        let original_text = "Hello, world!";

        let encrypted_result = decrypter.encrypt(original_text);
        assert!(encrypted_result.is_ok(), "Encryption failed");
        let encrypted_text = encrypted_result.unwrap();

        let decrypted_result = decrypter.decrypt(&encrypted_text);
        assert!(decrypted_result.is_ok(), "Decryption failed");
        let decrypted_text = decrypted_result.unwrap();

        assert_eq!(original_text, decrypted_text, "Original and decrypted text do not match");
    }
}
