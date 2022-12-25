use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};

pub fn encrypt(key: String, password: String) -> String {
    let mc = new_magic_crypt!(key, 256);

    mc.encrypt_str_to_base64(password)
}

pub fn decrypt(key: String, password: String) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(key, 256);

    mc.decrypt_base64_to_string(password)
}
