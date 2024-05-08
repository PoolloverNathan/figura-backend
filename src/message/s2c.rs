use super::MessageLoadError;
use std::convert::{TryFrom, TryInto};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S2CMessage<'a> {
    Auth = 0,
    Ping(u128, u8, bool, &'a [u8]) = 1,
    Event(u128) = 2,
    Toast(u8, &'a str, Option<&'a str>) = 3,
    Chat(&'a str) = 4,
    Notice(u8) = 5,
}
impl<'a> TryFrom<&'a [u8]> for S2CMessage<'a> {
    type Error = MessageLoadError;
    fn try_from(_: &'a [u8]) -> Result<Self, <Self as TryFrom<&'a [u8]>>::Error> {
        todo!()
    }
}
impl<'a> Into<&'a [u8]> for S2CMessage<'a> {
    fn into(self) -> &'a [u8] {
        todo!()
    }
}
