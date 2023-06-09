use serde::{Deserialize, Serialize};

use crate::enums::{JobCd, Method};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberIDResponse {
    #[serde(rename = "MemberID")]
    pub member_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMemberResponse {
    pub member_id: String,
    pub member_name: Option<String>,
    pub delete_flag: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveCardResponse {
    pub card_seq: String,
    pub card_no: String,
    pub forward: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteCardResponse {
    pub card_seq: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchCardResponse {
    pub card_seq: String,
    pub default_flag: String,
    pub card_name: String,
    pub card_no: String,
    pub expire: String,
    pub holder_name: String,
    pub delete_flag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTranResponse {
    #[serde(rename = "AccessID")]
    pub access_id: String,
    #[serde(rename = "AccessPass")]
    pub access_pass: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExecTranResponse {
    #[serde(rename = "ACS")]
    pub acs: String,
    #[serde(rename = "OrderID")]
    pub order_id: String,
    pub forward: String,
    pub method: Method,
    pub pay_times: String,
    pub approve: String,
    #[serde(rename = "TranID")]
    pub tran_id: String,
    pub tran_date: String,
    pub check_string: String,
    pub client_field1: Option<String>,
    pub client_field2: Option<String>,
    pub client_field3: Option<String>,
    pub acs_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterTranResponse {
    pub access_id: String,
    pub access_pass: String,
    pub forward: String,
    pub approve: String,
    pub tran_id: String,
    pub tran_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchTradeResponse {
    /// オーダーID
    pub order_id: String,
    /// 現状態
    pub status: String,
    /// 処理日時
    pub process_date: String,
    /// 処理区分
    pub job_cd: JobCd,
    /// 取引ID
    pub access_id: String,
    /// 取引パスワード
    pub access_pass: String,
    /// 商品コード
    pub item_code: String,
    /// 利用金額
    pub amount: String,
    /// 税送料
    pub tax: String,
    /// サイトID
    pub site_id: String,
    /// 会員ID
    pub member_id: String,
    /// カード番号
    pub card_no: String,
    /// 有効期限
    pub expire: String,
    /// 支払方法
    pub method: Method,
    /// 支払回数
    pub pay_times: String,
    /// 仕向先コード
    pub forward: String,
    /// トランザクションID
    pub tran_id: String,
    /// 承認番号
    pub approve: String,
    /// 加盟店自由項目1
    pub client_field1: String,
    /// 加盟店自由項目2
    pub client_field2: String,
    /// 加盟店自由項目3
    pub client_field3: String,
    /// エラーコード
    pub err_code: String,
    /// エラー詳細コード
    pub err_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeTranResponse {
    pub access_id: String,
    pub access_pass: String,
    pub forward: String,
    pub approve: String,
    pub tran_id: String,
    pub tran_date: String,
}
