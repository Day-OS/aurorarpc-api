use clap::{arg, Parser};
use log::info;
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use x360connect_global::activity;
use std::path::Path;
use std::vec;
mod errors;

use crate::connection::search_url;
use crate::database::game_data::{self};
use crate::database::verify_key::verify_key;
use crate::nova::authenticate::get_token;
use crate::nova::get_title::get_title;
use crate::rpc::{Activity, ActivityAssets, RPC};

mod nova;
mod connection;
mod database;
mod rpc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    key: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AppConfig{
    key: Option<String>,
    api_url: String,
}
impl Default for AppConfig{
    fn default() -> Self {
        Self { 
            api_url: "http://xbox.daytheipc.com".to_string(),
            key: None
        }
    }
}
impl AppConfig{
    pub fn save(&self, path: String) -> anyhow::Result<()>{
        let toml_str = toml::to_string_pretty(&self)?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }
}

async fn main_loop(rpc: &mut RPC, settings: &AppConfig) -> anyhow::Result<()>{
    let api_url = settings.api_url.clone();

    let url = search_url(vec![9999],None).await?;
        if let None = url{
            return Ok(());
        }
        let url = url.unwrap();
        log::info!("Xbox found at {}", url);
        let token = get_token(url.clone(), "xboxhttp", "1234").await?;

        log::info!("Signed in {}", url);

        let mut game_data: Option<activity::Activity>= None;
        let mut last_id: String = "".to_string();

        while let Ok(title) = get_title(url.clone(), token.clone()).await {
            match game_data {
                Some(ref data) => {
                    let data = data.clone();
                    rpc.start(
                        Activity{
                            //state: Some("???".to_string()),
                            details: Some(data.title.clone()),
                            assets: Some(ActivityAssets{
                                large_image: Some(data.icon.clone()),
                                //small_image: Some("http://xbox.daytheipc.com/assets/icons/4A5707D2.png".to_string()),
                                ..Default::default()

                            }),
                            ..Default::default()
                        }
                    ).await?; 
                    if title.titleid != last_id{
                        game_data = None;
                    }

                },
                None => {
                    let data = game_data::get_game_information(title.titleid.clone(), api_url.clone()).await;

                    game_data = Some(
                        match data {
                            Ok(data) => data,
                            Err(e) => {
                                log::error!("{e}");
                                activity::Activity{
                                    icon:"xbox-360-logo".to_string(),
                                    title:"".to_string(),
                                    player: None
                                
                                }
                            },
                        }
                    );
                    last_id = title.titleid.clone();
                    rpc.stop().await?;
                },
            }
        }
    rpc.stop().await?;
    Ok(())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();


    let config_path = "./config.toml";
    let config_p = Path::new(config_path);
    if !config_p.exists(){
        let default_config = AppConfig::default();
        default_config.save(config_path.to_string())?;
    }

    let settings = config::Config::builder()
        .add_source(config::File::with_name("./config.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    
    let mut settings: AppConfig = settings.try_deserialize::<AppConfig>()?;
    let args = Args::parse();

    if let Some(key) = args.key{
        match verify_key(settings.api_url.clone(), key.clone()).await? {
            true => {
                settings.key = Some(key.clone());
                settings.save(config_path.to_string())?;
                info!("Key {key} registered!");
            },
            false => {
                info!("Invalid key. Make sure you copied it correctly");
            },
        }
        return Ok(())
    }

    // In case the user has an outdated key that does not work anymore
    if let Some(key) = settings.key.clone(){
        match verify_key(settings.api_url.clone(), key).await {
            Ok(was_successful) => {
                if !was_successful{
                    settings.key = None;
                    settings.save(config_path.to_string())?;
                    return Err(anyhow::anyhow!("The current key seems to be either invalid or expired. Please, insert a new key using the '--key' parameter! "))
                }
            },
            Err(e) => {
                settings.key = None;
                settings.save(config_path.to_string())?;
                return Err(anyhow::anyhow!("{e}"))
            },
        };
    }

    if settings.key.is_none() && args.key.is_none(){
        info!("NO KEY FOUND!");
        info!(
            "Running this software without being logged may not show some images from games that \
            were not included in the database before. It won't also show any information about your \
            achievements, nor your profile."
        );
    }

    let mut rpc = rpc::RPC::new().await;
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

    loop {
        main_loop(&mut rpc, &settings).await?;

    }
}
