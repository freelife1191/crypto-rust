extern crate ring;

use crate::error::crypto_error::{CryptoError, CryptoResult};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, Key, KeyIvInit};
use aes::{Aes128, Aes192, Aes256};
// AES-GCM 구조체 및 관련 타입
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::prelude::*;
use block_padding::array::typenum::Unsigned;
use cbc::{Decryptor, Encryptor};
use generic_array::ArrayLength;
use md5;
use rand::RngCore;
use ring::digest;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(unused)]
type Aes128CbcEnc = Encryptor<Aes128>;
#[allow(unused)]
type Aes128CbcDec = Decryptor<Aes128>;
#[allow(unused)]
type Aes192CbcEnc = Encryptor<Aes192>;
#[allow(unused)]
type Aes192CbcDec = Decryptor<Aes192>;
type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

#[allow(unused)]
#[derive(Default, Clone, Copy, Debug)]
pub enum Algorithm {
    AES128,
    AES192,
    #[default]
    AES256,
}

#[allow(unused)]
impl Algorithm {
    fn key_len(&self) -> usize {
        match self {
            Algorithm::AES128 => 16,
            Algorithm::AES192 => 24,
            Algorithm::AES256 => 32,
        }
    }
}

#[allow(unused)]
trait KeySize {
    type Size: ArrayLength;
    fn key_len(&self) -> usize;
}

#[allow(unused)]
pub fn rand_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    // let mut rng = OsRng;
    let mut rng = rand::rng();
    rng.fill_bytes(&mut bytes);
    bytes
}

// pub(crate) fn digest(alg: &str, data: &[u8], count: usize) -> Result<Vec<u8>, String> {
pub fn digest(alg: &str, data: &[u8], count: i32) -> CryptoResult<Vec<u8>> {
    let mut ret = data.to_vec();
    for _ in 0..count {
        ret = match alg {
            "SHA-256" => digest::digest(&digest::SHA256, &ret).as_ref().to_vec(),
            "MD5" => md5::compute(&ret).to_vec(),
            _ => return Err(CryptoError::UtilError(format!("Unsupported algorithm: {}", alg))),
        };
    }
    Ok(ret)
}

pub fn encrypt(algorithm: Algorithm, data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    match algorithm {
        Algorithm::AES128 => encrypt_aes_cbc::<Aes128CbcEnc>(data, key, iv),
        Algorithm::AES192 => encrypt_aes_cbc::<Aes192CbcEnc>(data, key, iv),
        Algorithm::AES256 => encrypt_aes_cbc::<Aes256CbcEnc>(data, key, iv),
    }
}

pub fn decrypt(algorithm: Algorithm, data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    match algorithm {
        Algorithm::AES128 => decrypt_aes_cbc::<Aes128CbcDec>(data, key, iv),
        Algorithm::AES192 => decrypt_aes_cbc::<Aes192CbcDec>(data, key, iv),
        Algorithm::AES256 => decrypt_aes_cbc::<Aes256CbcDec>(data, key, iv),
    }
}

pub fn encrypt_aes_cbc<T>(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>>
where T: BlockEncryptMut + KeyIvInit {
    if key.len() != T::KeySize::to_usize() {
        let err_message = format!("Invalid key length: expected {} bytes, got {}", T::KeySize::to_usize(), key.len());
        return Err(CryptoError::EncryptError(err_message));
    }

    if iv.len() != 16 {
        let err_message = format!("Invalid IV length: expected 16 bytes, got {}", iv.len());
        return Err(CryptoError::DecryptError(err_message));
    }

    let encryptor = T::new_from_slices(key, iv).unwrap();

    let mut padded_data = &data;
    // println!("key len: {:?}, iv len: {:?}, padded_data len: {:?}", key.len(), iv.len(), padded_data.len());
    let buf_size = padded_data.len() + 16;
    // println!("buf_size: {:?}", buf_size);
    let mut buf = vec![0u8; buf_size];
    // println!("buf len: {:?}", buf.len());

    let encrypt = encryptor.encrypt_padded_b2b_mut::<Pkcs7>(&mut padded_data, &mut buf)
        .map_err(|e| CryptoError::EncryptError(e.to_string()))?;
    Ok(encrypt.to_vec())
}

pub fn decrypt_aes_cbc<T>(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>>
// where 절을 이용해 Trait Bound를 지정 (Rust에서 다형성으로 지원하기 위해 사용 동적구성이 가능하나
// https://taegit.tistory.com/8
where T: BlockDecryptMut + KeyIvInit {
    if key.len() != T::KeySize::to_usize() {
        let err_message = format!("Invalid key length: expected {} bytes, got {}", T::KeySize::to_usize(), key.len());
        return Err(CryptoError::DecryptError(err_message));
    }

    if iv.len() != 16 {
        let err_message = format!("Invalid key length: expected {} bytes, got {}", T::KeySize::to_usize(), key.len());
        return Err(CryptoError::DecryptError(err_message));
    }

    let decryptor = T::new_from_slices(key, iv).unwrap();

    let padded_data = &data;
    // println!("key len: {:?}, iv len: {:?}, padded_data len: {:?}", key.len(), iv.len(), padded_data.len());
    let buf_size = padded_data.len();
    // println!("buf_size: {:?}", buf_size);
    let mut buf = vec![0u8; buf_size];
    // println!("buf len: {:?}", buf.len());

    let decrypt = decryptor.decrypt_padded_b2b_mut::<Pkcs7>(&padded_data, &mut buf)
        .map_err(|e| CryptoError::DecryptError(e.to_string()))?;
    Ok(decrypt.to_vec())
}

pub fn encrypt_algorithm(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    get_algorithm(key).and_then(|alg| Ok(encrypt(alg, data, key, iv)))?
}

pub fn decrypt_algorithm(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    get_algorithm(key).and_then(|alg| Ok(decrypt(alg, data, key, iv)))?
}

fn get_algorithm(key: &[u8]) -> CryptoResult<Algorithm> {
    let algorithm = if key.len() == 16 {
        Algorithm::AES128
    } else if key.len() == 24 {
        Algorithm::AES192
    } else if key.len() == 32 {
        Algorithm::AES256
    } else {
        let err_message = format!("Invalid key length: expected 16, 24, or 32 bytes, got {}", key.len());
        return Err(CryptoError::EncryptError(err_message));
    };
    // println!("Algorithm: {:?}", algorithm);
    Ok(algorithm)
}

pub fn encode_base64(data: &[u8]) -> String {
    BASE64_STANDARD.encode(data)
}

// pub fn encrypt_gcm(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
//     let cipher = ring::aead::AES_256_GCM;
//     let key = ring::aead::UnboundKey::new(&cipher, key)
//         .map_err(|e| CryptoError::EncryptError(e.to_string()))?;
// }

#[allow(unused)]
pub fn decode_base64(data: &str) -> Vec<u8> {
    BASE64_STANDARD.decode(data).unwrap()
}

#[allow(unused)]
pub fn encode_aes_256_gcm(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    // 1. 키 생성 (32바이트, AES-256)
    // let mut key = [0u8; 32];
    // rand::thread_rng().fill_bytes(&mut key_bytes);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    // 2. 랜덤 Nonce 생성 (12바이트)
    // let mut nonce_bytes = [0u8; 12];
    let nonce_bytes = &iv[..12];
    // rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 3. 평문 (Plaintext) 정의
    // let plaintext = "안녕하세요! AES-GCM-256 암호화 예제입니다.".as_bytes();

    // 4. 암호화
    let ciphertext = cipher.encrypt(nonce, data.as_ref())
        .map_err(|e| CryptoError::EncryptError(e.to_string()))?;
    Ok(ciphertext.to_vec())
}

#[allow(unused)]
pub fn decode_aes_256_gcm(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
    // 1. 키 생성 (32바이트, AES-256)
    // let mut key = [0u8; 32];
    // rand::thread_rng().fill_bytes(&mut key_bytes);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    // 2. 랜덤 Nonce 생성 (12바이트)
    // let mut nonce_bytes = [0u8; 12];
    let nonce_bytes = &iv[..12];
    // rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 3. 평문 (Plaintext) 정의
    // let plaintext = "안녕하세요! AES-GCM-256 암호화 예제입니다.".as_bytes();

    // 4. 복호화
    // println!("복호화된 데이터: {}", String::from_utf8_lossy(&decrypted_text));
    // println!("data: {:?}", encode_base64(data));
    let plaintext = cipher.decrypt(nonce, data.as_ref())
        .map_err(|e| CryptoError::DecryptError(e.to_string()))?;
    Ok(plaintext.to_vec())
}

#[allow(unused)]
pub fn to_json_bytes<T: Serialize>(o: &T) -> Vec<u8> {
    serde_json::to_vec(o).unwrap()
}

/*
pub fn from_json_file<T: for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> T {
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}
*/

#[allow(unused)]
pub fn from_json_byte<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Box<dyn Error>> {
    let config: T = serde_json::from_slice(&data)
        .map_err(|e| CryptoError::ConfigParsingError(e.to_string()))?;
    Ok(config)
}

#[allow(unused)]
pub fn from_json_path<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    // let path_buf = path::absolute(path.to_str()).expect("Unable to get absolute path");
    // let x = path_buf.as_path();
    let mut file = File::open(path.as_os_str())
        .map_err(|e| CryptoError::ConfigFileError(e.to_string()))?;
    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| CryptoError::ConfigPathError(e.to_string()))?;
    let config: T = serde_json::from_str(&data)
        .map_err(|e| CryptoError::ConfigParsingError(e.to_string()))?;
    Ok(config)
}

#[allow(unused)]
pub fn from_json_file<T: for<'de> Deserialize<'de>>(file: &mut File) -> Result<T, Box<dyn Error>> {
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let config: T = serde_json::from_str(&data)?;
    Ok(config)
}