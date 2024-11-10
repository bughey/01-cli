use anyhow::Result;
use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;

use crate::cli::genpass::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(opts: GenPassOpts) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    for _ in 0..(opts.length - password.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        // password.push(chars[idx] as char);

        password.push(*chars.choose(&mut rng).unwrap());
    }
    password.shuffle(&mut rng);

    let pwd = String::from_utf8(password)?;
    print!("{}", pwd);

    let estimate = zxcvbn(&pwd, &[]);
    eprintln!("Password strength {}", estimate.score());

    Ok(())
}
