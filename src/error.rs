use neovim_lib::CallError;
use snafu::prelude::*;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Nvim command {} occur error, source: {}", name, source))]
    NvimCommand { name: String, source: CallError },
}
