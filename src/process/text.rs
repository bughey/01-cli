use std::{fs, io::Read};

use anyhow::Result;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{
    cli::text::{TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts},
    utils::get_reader,
};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

#[allow(dead_code)]
struct Ed25519Signer {
    key: SigningKey,
}

#[allow(dead_code)]
struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text(subcmd: TextSubCommand) -> Result<()> {
    match subcmd {
        TextSubCommand::Sign(opts) => process_sign(opts),
        TextSubCommand::Verify(opts) => process_verify(opts),
    }
}

fn process_sign(opts: TextSignOpts) -> Result<()> {
    println!("{:?}", opts);

    let mut reader = get_reader(opts.input.as_str())?;

    let signed = match opts.format {
        TextSignFormat::Blake3 => {
            let key = fs::read(opts.key.as_str())?;
            let key = key.try_into().unwrap();
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            // sign with ed25519
            todo!()
        }
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);
    println!("\nsigned: {}", signed);
    Ok(())
}

fn process_verify(opts: TextVerifyOpts) -> Result<()> {
    println!("{:?}", opts);

    let reader = get_reader(opts.input.as_str())?;

    match opts.format {
        TextSignFormat::Blake3 => {
            let key = fs::read(opts.key.as_str())?;
            let key = key.try_into().unwrap();
            let verifier = Blake3 { key };
            let sig = BASE64_URL_SAFE_NO_PAD.decode(opts.sig.as_bytes())?;
            let verified = verifier.verify(reader, &sig)?;
            println!("\nverified: {}", verified);
        }
        TextSignFormat::Ed25519 => {
            // verify with ed25519
            todo!()
        }
    }

    Ok(())
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

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
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
