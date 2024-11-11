use std::{fs, io::Read, path::Path};

use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{
    cli::{
        genpass::GenPassOpts,
        text::{TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts},
    },
    utils::get_reader,
};

use super::gen_pass::process_genpass;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    // TODO use GAT alternative
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = key.try_into().unwrap();
        Ok(Self::new(key))
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes() == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(GenPassOpts {
            length: 32,
            uppercase: true,
            lowercase: true,
            number: true,
            symbol: true,
        })?;
        let key = vec![key.into_bytes()];
        Ok(key)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key();
        let skey = sk.as_bytes().to_vec();
        let pkey = pk.as_bytes().to_vec();
        Ok(vec![skey, pkey])
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub fn process_text(subcmd: TextSubCommand) -> Result<()> {
    match subcmd {
        TextSubCommand::Sign(opts) => {
            let sig = process_sign(opts)?;
            println!("\nsigned: {}", sig);
        }
        TextSubCommand::Verify(opts) => {
            let verified = process_verify(opts)?;
            println!("\nverified: {}", verified);
        }
        TextSubCommand::Generate(opts) => {
            let keys = process_generate(&opts)?;

            match opts.format {
                TextSignFormat::Blake3 => {
                    let name = opts.output.join("blake3.txt");
                    fs::write(name, &keys[0])?;
                }
                TextSignFormat::Ed25519 => {
                    let name = opts.output.join("ed25519");
                    if !name.exists() {
                        fs::create_dir_all(&name)?;
                    }
                    fs::write(name.join("sk"), &keys[0])?;
                    fs::write(name.join("pk"), &keys[1])?;
                }
            }
        }
    }
    Ok(())
}

fn process_generate(opts: &TextKeyGenerateOpts) -> Result<Vec<Vec<u8>>> {
    println!("{:?}", opts);

    match opts.format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

fn process_sign(opts: TextSignOpts) -> Result<String> {
    let mut reader = get_reader(opts.input.as_str())?;

    let signed = match opts.format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(opts.key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(opts.key)?;
            signer.sign(&mut reader)?
        }
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

fn process_verify(opts: TextVerifyOpts) -> Result<bool> {
    println!("{:?}", opts);

    let reader = get_reader(opts.input.as_str())?;

    let sig = BASE64_URL_SAFE_NO_PAD.decode(opts.sig.as_bytes())?;
    let verified = match opts.format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(opts.key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(opts.key)?;
            verifier.verify(reader, &sig)?
        }
    };

    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519/sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519/pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut data.as_ref())?;
        assert!(pk.verify(data.as_ref(), &sig)?);

        Ok(())
    }
}
