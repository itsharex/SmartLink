use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
use crate::error::Error;

pub struct KeyPair {
    pub private_key: EphemeralSecret,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>, // 加密后的内容
    pub nonce: Vec<u8>,      // 随机数
}

pub struct Encryption {
    rng: OsRng, // 安全的随机数生成器
}

impl Encryption {
    pub fn new() -> Self {
        Encryption { rng: OsRng }
    }

    // 生成ECDH密钥对
    pub fn generate_key_pair(&mut self) -> Result<KeyPair, Error> {
        let private_key = EphemeralSecret::random_from_rng(&mut self.rng);
        let public_key = PublicKey::from(&private_key);
        Ok(KeyPair {
            private_key,
            public_key,
        })
    }

    // 计算共享密钥，消耗 private_key
    pub fn derive_shared_secret(
        &self,
        private_key: EphemeralSecret,
        public_key: &PublicKey,
    ) -> Result<SharedSecret, Error> {
        Ok(private_key.diffie_hellman(public_key))
    }

    // 加密消息
    pub fn encrypt_message(
        &mut self,
        plaintext: &str,
        shared_secret: &SharedSecret,
    ) -> Result<EncryptedMessage, Error> {
        let key = Aes256Gcm::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| Error::Encrypt(e.to_string()))?;
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill_bytes(&mut nonce_bytes); // 每次生成唯一的Nonce
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = key
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| Error::Encrypt(e.to_string()))?;
        Ok(EncryptedMessage {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    // 解密消息
    pub fn decrypt_message(
        &self,
        encrypted: &EncryptedMessage,
        shared_secret: &SharedSecret,
    ) -> Result<String, Error> {
        let key = Aes256Gcm::new_from_slice(shared_secret.as_bytes())
            .map_err(|e| Error::Decrypt(e.to_string()))?;
        if encrypted.nonce.len() != 12 {
            return Err(Error::InvalidNonce);
        }
        let nonce = Nonce::from_slice(&encrypted.nonce);
        let plaintext = key
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| Error::Decrypt(e.to_string()))?;
        String::from_utf8(plaintext)
            .map_err(|e| Error::Decrypt(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64;

    // 测试端到端加密的完整流程
    #[test]
    fn test_end_to_end_encryption() {
        let mut encryption = Encryption::new();

        // 模拟Alice和Bob的密钥对生成
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");

        // Alice的公钥和Bob的公钥（假设已交换）
        let alice_public = &alice_keys.public_key;
        let bob_public = &bob_keys.public_key;

        // Alice计算共享密钥
        let alice_shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, bob_public)
            .expect("Alice shared secret derivation failed");
        // Bob计算共享密钥
        let bob_shared_secret = encryption
            .derive_shared_secret(bob_keys.private_key, alice_public)
            .expect("Bob shared secret derivation failed");

        // 验证共享密钥是否相同
        assert_eq!(
            alice_shared_secret.as_bytes(),
            bob_shared_secret.as_bytes(),
            "Shared secrets do not match"
        );

        // Alice加密消息
        let plaintext = "我的银行密码是123456";
        let encrypted_message = encryption
            .encrypt_message(plaintext, &alice_shared_secret)
            .expect("Encryption failed");

        // 输出加密结果（供调试）
        let ciphertext_base64 = base64::encode(&encrypted_message.ciphertext);
        let nonce_base64 = base64::encode(&encrypted_message.nonce);
        println!("Encrypted ciphertext: {}", ciphertext_base64);
        println!("Nonce: {}", nonce_base64);

        // 验证密文不是明文
        assert_ne!(
            String::from_utf8_lossy(&encrypted_message.ciphertext),
            plaintext,
            "Ciphertext should not match plaintext"
        );

        // Bob解密消息
        let decrypted = encryption
            .decrypt_message(&encrypted_message, &bob_shared_secret)
            .expect("Decryption failed");

        // 验证解密结果
        assert_eq!(
            decrypted, plaintext,
            "Decrypted message does not match original"
        );
        println!("Decrypted message: {}", decrypted);
    }

    // 测试使用错误密钥解密
    #[test]
    fn test_decryption_with_wrong_key() {
        let mut encryption = Encryption::new();

        // 生成Alice和Bob的密钥对
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");

        // Alice加密消息
        let plaintext = "我的银行密码是123456";
        let alice_shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Alice shared secret derivation failed");
        let encrypted_message = encryption
            .encrypt_message(plaintext, &alice_shared_secret)
            .expect("Encryption failed");

        // 使用错误的密钥尝试解密
        let mut wrong_encryption = Encryption::new();
        let wrong_keys = wrong_encryption
            .generate_key_pair()
            .expect("Wrong key generation failed");
        let wrong_shared_secret = wrong_encryption
            .derive_shared_secret(wrong_keys.private_key, &alice_keys.public_key)
            .expect("Wrong shared secret derivation failed");

        let result = encryption.decrypt_message(&encrypted_message, &wrong_shared_secret);
        assert!(result.is_err(), "Decryption with wrong key should fail");
        if let Err(e) = result {
            println!("Expected error with wrong key: {}", e);
            assert!(
                e.to_string().contains("Decrypt"),
                "Error should be related to decryption failure"
            );
        }
    }

    // 测试无效Nonce
    #[test]
    fn test_invalid_nonce() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 创建一个无效的EncryptedMessage（Nonce长度不对）
        let invalid_encrypted = EncryptedMessage {
            ciphertext: vec![0; 16],
            nonce: vec![0; 8], // Nonce应为12字节，这里故意设为8字节
        };

        let result = encryption.decrypt_message(&invalid_encrypted, &shared_secret);
        assert!(result.is_err(), "Decryption with invalid nonce should fail");
        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "Invalid nonce",
                "Error should be InvalidNonce"
            );
            println!("Expected error with invalid nonce: {}", e);
        }
    }

    // 测试空消息加密和解密
    #[test]
    fn test_empty_message() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 加密空消息
        let plaintext = "";
        let encrypted_message = encryption
            .encrypt_message(plaintext, &shared_secret)
            .expect("Encryption of empty message failed");

        // 验证密文不为空（包含认证标签）
        assert!(!encrypted_message.ciphertext.is_empty(), "Ciphertext should not be empty even for empty plaintext");

        // 解密空消息
        let decrypted = encryption
            .decrypt_message(&encrypted_message, &shared_secret)
            .expect("Decryption of empty message failed");

        assert_eq!(decrypted, plaintext, "Decrypted empty message should match original");
        println!("Decrypted empty message: '{}'", decrypted);
    }

    // 测试超长消息加密和解密
    #[test]
    fn test_long_message() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 创建一个超长消息（10KB）
        let plaintext = "A".repeat(10 * 1024);
        let encrypted_message = encryption
            .encrypt_message(&plaintext, &shared_secret)
            .expect("Encryption of long message failed");

        // 验证密文长度合理
        assert!(
            encrypted_message.ciphertext.len() >= plaintext.len(),
            "Ciphertext should be at least as long as plaintext plus tag"
        );

        // 解密超长消息
        let decrypted = encryption
            .decrypt_message(&encrypted_message, &shared_secret)
            .expect("Decryption of long message failed");

        assert_eq!(decrypted, plaintext, "Decrypted long message should match original");
        println!("Successfully decrypted long message of length: {}", decrypted.len());
    }

    // 测试Nonce的唯一性
    #[test]
    fn test_nonce_uniqueness() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 多次加密同一消息，检查Nonce是否不同
        let plaintext = "Test nonce uniqueness";
        let encrypted1 = encryption
            .encrypt_message(plaintext, &shared_secret)
            .expect("First encryption failed");
        let encrypted2 = encryption
            .encrypt_message(plaintext, &shared_secret)
            .expect("Second encryption failed");

        assert_ne!(
            encrypted1.nonce, encrypted2.nonce,
            "Nonces should be unique for each encryption"
        );
        println!(
            "Nonce 1: {}, Nonce 2: {}",
            base64::encode(&encrypted1.nonce),
            base64::encode(&encrypted2.nonce)
        );

        // 验证解密仍然正确
        let decrypted1 = encryption
            .decrypt_message(&encrypted1, &shared_secret)
            .expect("Decryption of first message failed");
        let decrypted2 = encryption
            .decrypt_message(&encrypted2, &shared_secret)
            .expect("Decryption of second message failed");

        assert_eq!(decrypted1, plaintext, "First decrypted message should match original");
        assert_eq!(decrypted2, plaintext, "Second decrypted message should match original");
    }

    // 测试篡改密文后的解密失败
    #[test]
    fn test_tampered_ciphertext() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 加密消息
        let plaintext = "Sensitive data";
        let mut encrypted_message = encryption
            .encrypt_message(plaintext, &shared_secret)
            .expect("Encryption failed");

        // 篡改密文（翻转第一个字节）
        if !encrypted_message.ciphertext.is_empty() {
            encrypted_message.ciphertext[0] ^= 0xFF;
        }

        // 尝试解密篡改后的密文
        let result = encryption.decrypt_message(&encrypted_message, &shared_secret);
        assert!(result.is_err(), "Decryption of tampered ciphertext should fail");
        if let Err(e) = result {
            println!("Expected error with tampered ciphertext: {}", e);
            assert!(
                e.to_string().contains("Decrypt"),
                "Error should be related to decryption failure"
            );
        }
    }

    // 测试篡改Nonce后的解密失败
    #[test]
    fn test_tampered_nonce() {
        let mut encryption = Encryption::new();

        // 生成密钥对和共享密钥
        let alice_keys = encryption.generate_key_pair().expect("Alice key generation failed");
        let bob_keys = encryption.generate_key_pair().expect("Bob key generation failed");
        let shared_secret = encryption
            .derive_shared_secret(alice_keys.private_key, &bob_keys.public_key)
            .expect("Shared secret derivation failed");

        // 加密消息
        let plaintext = "Sensitive data";
        let mut encrypted_message = encryption
            .encrypt_message(plaintext, &shared_secret)
            .expect("Encryption failed");

        // 篡改Nonce（翻转第一个字节）
        if !encrypted_message.nonce.is_empty() {
            encrypted_message.nonce[0] ^= 0xFF;
        }

        // 尝试解密篡改后的消息
        let result = encryption.decrypt_message(&encrypted_message, &shared_secret);
        assert!(result.is_err(), "Decryption with tampered nonce should fail");
        if let Err(e) = result {
            println!("Expected error with tampered nonce: {}", e);
            assert!(
                e.to_string().contains("Decrypt"),
                "Error should be related to decryption failure"
            );
        }
    }
}