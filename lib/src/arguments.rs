use crate::enums::{JobCd, Method, SeqMode, UseFloatingMask, ValidFlag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MemberArgs {
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
    pub member_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMemberArgs {
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveCardArgs {
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
    pub seq_mode: Option<SeqMode>,
    pub card_seq: Option<u32>,
    pub default_flag: Option<String>,
    pub card_name: Option<String>,
    pub card_no: Option<String>,
    pub card_pass: Option<String>,
    pub expire: Option<String>,
    pub holder_name: Option<String>,
    pub security_code: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteCardArgs {
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
    pub seq_mode: Option<SeqMode>,
    pub card_seq: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchCardArgs {
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
    pub seq_mode: Option<SeqMode>,
    pub valid_flag: Option<ValidFlag>,
    pub card_seq: Option<u32>,
    pub use_floating_mask: Option<UseFloatingMask>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradedCardArgs {
    #[serde(rename = "ShopID")]
    pub shop_id: String,
    pub shop_pass: String,
    #[serde(rename = "OrderID")]
    pub order_id: String,
    #[serde(rename = "SiteID")]
    pub site_id: String,
    pub site_pass: String,
    #[serde(rename = "MemberID")]
    pub member_id: String,
    pub seq_mode: Option<SeqMode>,
    pub card_seq: Option<u32>,
    pub default_flag: Option<String>,
    pub card_name: Option<String>,
    pub card_pass: Option<String>,
    pub holder_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalSiteArgs {
    pub site_id: Option<String>,
    pub site_pass: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntryTranArgs {
    #[serde(rename = "ShopID")]
    pub shop_id: String,
    pub shop_pass: String,
    #[serde(rename = "OrderID")]
    pub order_id: String,
    pub job_cd: JobCd,
    pub amount: u32,
    pub tax: Option<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecTranArgs {
    #[serde(rename = "AccessID")]
    pub access_id: String,
    pub access_pass: String,
    #[serde(rename = "OrderID")]
    pub order_id: String,
    pub method: Option<Method>,
    pub pay_times: Option<u32>,
    pub card_no: Option<String>,
    pub expire: Option<String>,
    pub security_code: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "PIN")]
    pub pin: Option<String>,
    #[serde(rename = "SiteID")]
    pub site_id: Option<String>,
    pub site_pass: Option<String>,
    #[serde(rename = "MemberID")]
    pub member_id: Option<String>,
    pub seq_mode: Option<SeqMode>,
    pub card_seq: Option<u32>,
    pub card_pass: Option<String>,
    pub client_field1: Option<String>,
    pub client_field2: Option<String>,
    pub client_field3: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
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
#[serde(rename_all = "PascalCase")]
pub struct SearchTradeArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeTranArgs {
    pub shop_id: String,
    pub shop_pass: String,
    pub access_id: String,
    pub access_pass: String,
    pub job_cd: JobCd,
    pub amount: u32,
    pub tax: Option<String>,
}
