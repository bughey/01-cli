use anyhow::Result;

use crate::cli::http::HttpSubCommand;

pub fn process_http(subcmd: HttpSubCommand) -> Result<()> {
    match subcmd {
        HttpSubCommand::Serve(opts) => {
            println!(
                "Direct [{}] Serve at http://0.0.0.0:{}",
                opts.dir.display(),
                opts.port
            );
        }
    }
    Ok(())
}
