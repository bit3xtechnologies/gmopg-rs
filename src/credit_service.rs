

use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use crate::arguments::{AlterTranArgs, ChangeTranArgs, EntryTranArgs, ExecTranArgs, SearchTradeArgs};
use crate::response::{AlterTranResponse, ChangeTranResponse, EntryTranResponse, ExecTranResponse, SearchTradeResponse};

pub struct CreditService {
    client: reqwest::Client,
    base_url: String,
}

impl CreditService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn entry_tran(&self, args: EntryTranArgs) -> Result<EntryTranResponse, reqwest::Error> {
        self.post("EntryTran.idPass", args).await
    }

    pub async fn exec_tran(&self, args: ExecTranArgs) -> Result<ExecTranResponse, reqwest::Error> {
        self.post("ExecTran.idPass", args).await
    }

    pub async fn alter_tran(&self, args: AlterTranArgs) -> Result<AlterTranResponse, reqwest::Error> {
        self.post("AlterTran.idPass", args).await
    }

    pub async fn search_trade(&self, args: SearchTradeArgs) -> Result<SearchTradeResponse, reqwest::Error> {
        self.post("SearchTrade.idPass", args).await
    }

    pub async fn change_tran(&self, args: ChangeTranArgs) -> Result<ChangeTranResponse, reqwest::Error> {
        self.post("ChangeTran.idPass", args).await
    }

    // pub async fn search_card_detail(&self, args: SearchCardDetailArgs) -> Result<SearchCardDetailResult, reqwest::Error> {
    //     self.post("SearchCardDetail.idPass", args).await
    // }

    async fn post<T: Serialize, R: DeserializeOwned>(&self, action: &str, args: T) -> Result<R, reqwest::Error> {
        let url = format!("{}/payment/{}", self.base_url, action);
        let res = self.client.post(&url).json(&args).send().await?;
        let response = res.json().await?;
        Ok(response)
    }
}