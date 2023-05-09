use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::arguments::{DeleteMemberArgs, MemberArgs};
use crate::response::{MemberIDResponse, SearchMemberResponse};

#[derive(Debug, Clone)]
pub struct CardService {
    client: Client,
    base_url: String,
}

impl CardService {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn save_member(&self, args: MemberArgs) -> Result<MemberIDResponse, reqwest::Error> {
        self.post("SaveMember", args).await
    }

    pub async fn update_member(
        &self,
        args: MemberArgs,
    ) -> Result<MemberIDResponse, reqwest::Error> {
        self.post("UpdateMember", args).await
    }

    pub async fn delete_member(
        &self,
        args: DeleteMemberArgs,
    ) -> Result<MemberIDResponse, reqwest::Error> {
        self.post("DeleteMember", args).await
    }

    pub async fn search_member(
        &self,
        args: MemberArgs,
    ) -> Result<SearchMemberResponse, reqwest::Error> {
        self.post("SearchMember", args).await
    }

    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        args: T,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}/payment/{}.idPass", self.base_url, action);
        let res = self.client.post(&url).json(&args).send().await?;
        let response = res.json().await?;
        Ok(response)
    }
}
