#[derive(Debug)]
pub enum Error {
    InvalidCard { msg: String },
    ConnectionError { msg: String },
}

impl Error {
    pub fn invalid_card(msg: String) -> Error {
        Error::InvalidCard { msg }
    }

    pub fn connection_error(msg: String) -> Error {
        Error::ConnectionError { msg }
    }
}
