use serde::{Deserialize, Serialize};
use crate::enums::{JobCd, Method, SeqMode};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberArgs {
    pub site_id: String,
    pub site_pass: String,
    pub member_id: String,
    pub member_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMemberArgs {
    pub site_id: String,
    pub site_pass: String,
    pub member_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalSiteArgs {
    pub site_id: Option<String>,
    pub site_pass: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTranArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub order_id: String,
    pub job_cd: JobCd,
    pub amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecTranArgs {
    pub access_id: String,
    pub access_pass: String,
    pub order_id: String,
    pub method: Option<Method>,
    pub pay_times: Option<u32>,
    pub card_no: Option<String>,
    pub expire: Option<String>,
    pub security_code: Option<String>,
    pub token: Option<String>,
    pub pin: Option<String>,
    pub site_id: Option<String>,
    pub site_pass: Option<String>,
    pub member_id: Option<String>,
    pub seq_mode: Option<SeqMode>,
    pub card_seq: Option<u32>,
    pub card_pass: Option<String>,
    pub client_field1: Option<String>,
    pub client_field2: Option<String>,
    pub client_field3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterTranArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub access_id: String,
    pub access_pass: String,
    pub job_cd: JobCd,
    pub amount: Option<u32>,
    pub method: Option<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  SearchTradeArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTranArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub access_id: String,
    pub access_pass: String,
    pub job_cd: JobCd,
    pub amount: u32,
    pub tax: Option<String>,
}