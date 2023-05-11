use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum SeqMode {
    Logic = 0,
    Physics = 1,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DefaultFlag {
    BillingObject = 0,
    NotSubjectToCharge = 1,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PayType {
    Cash = 0,
    Credit = 1,
    Suica = 2,
    Edy = 3,
    Cvs = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum Method {
    Lump = 1,
    Installment = 2,
    BonusLump = 3,
    Revolving = 4,
    BonusInstallment = 5,
}

impl Default for Method {
    fn default() -> Self {
        Self::Lump
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum JobCd {
    Check,
    Capture,
    Auth,
    Sales,
    Void,
    Return,
    Returnx,
    Sauth,
}

impl Default for JobCd {
    fn default() -> Self {
        Self::Auth
    }
}
