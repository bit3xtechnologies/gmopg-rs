use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SeqMode {
    Logic = 0,
    Physics = 1,
}

#[derive(Debug, Serialize, Deserialize,  PartialEq, Eq)]
pub enum DefaultFlag {
    BillingObject = 0,
    NotSubjectToCharge = 1,
}

#[derive(Debug, Serialize, Deserialize,  PartialEq, Eq)]
pub enum PayType {
    Cash = 0,
    Credit = 1,
    Suica = 2,
    Edy = 3,
    Cvs = 4,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Lump,
    Installment,
    BonusLump,
    Revolving,
    BonusInstallment,
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
