use anyhow::Ok;
use yet_another_discord_rpc::DiscordRpc;

use crate::errors::AuroraRpcError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActivityTimestamps {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    pub(crate) icon: String,
    pub(crate) title: String,
}

pub struct RPC{
    base: DiscordRpc,
    is_started: bool,
}

impl RPC{
    pub async fn new() -> RPC{
        const CLIENT_ID: &str = "1362896414044065832";
        let rpc = DiscordRpc::new(CLIENT_ID).await.unwrap();
        RPC{
            base: rpc,
            is_started: false
        }
    }

    pub async fn start(&mut self, activity: Activity) -> anyhow::Result<()> {
        let activity = serde_json::to_value(&activity).unwrap();

        if self.is_started{
            self.base.set_activity(activity).await.map_err(|e| AuroraRpcError::RPCError(e.to_string()))?;
        }
        else{
            self.base.start_activity(Some(activity)).await.map_err(|e| AuroraRpcError::RPCError(e.to_string()))?;
        }
        
        self.is_started = true;

        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.is_started = false;
        self.base.stop_activity().await.map_err(|e| AuroraRpcError::RPCError(e.to_string()))?;
        Ok(())
    }

}