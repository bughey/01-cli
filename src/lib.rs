pub mod cli;
pub mod process;
pub mod utils;

use anyhow::Result;
use enum_dispatch::enum_dispatch;

use crate::cli::base64::*;
use crate::cli::csv::*;
use crate::cli::genpass::*;
use crate::cli::http::*;
use crate::cli::text::*;
use crate::cli::*;

// rcli csv -i input.csv -o output.json --header -d ','

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait Processor {
    async fn process(self) -> Result<()>;
}
