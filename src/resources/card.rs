use crate::error::Error;

#[derive(Copy, Clone, Debug)]
pub struct CardId(u8);

impl TryFrom<u8> for CardId {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self, Error> {
        if let 1..=78 = val {
            Ok(Self(val))
        } else {
            Err(Error::invalid_card("Tarot have only 78 cards".into()))
        }
    }
}

impl Into<u8> for CardId {
    fn into(self) -> u8 {
        self.0
    }
}
