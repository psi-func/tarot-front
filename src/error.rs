#[derive(Debug)]
pub enum Error {
    InvalidCard { msg: String },
}

impl Error {
    pub fn invalid_card(msg: String) -> Error {
        Error::InvalidCard { msg }
    }
}
