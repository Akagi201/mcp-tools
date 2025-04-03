use std::{collections::HashMap, path::PathBuf, process::Stdio};

use clap::Parser;
use config::{Config as FileConfig, ConfigError, Environment, File};
use eyre::Result;
use rmcp::{RoleClient, ServiceExt, service::RunningService};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "protocol", rename_all = "lowercase")]
pub enum McpServerTransportConfig {
    Sse {
        url: String,
    },
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        envs: HashMap<String, String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    name: String,
    #[serde(flatten)]
    transport: McpServerTransportConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    server: McpServerConfig,
}

pub type McpClient = RunningService<RoleClient, ()>;

impl McpConfig {
    pub async fn create_client(&self) -> eyre::Result<McpClient> {
        let client = self.server.transport.start().await?;
        Ok(client)
    }
}

impl McpServerTransportConfig {
    pub async fn start(&self) -> eyre::Result<McpClient> {
        let client = match self {
            McpServerTransportConfig::Sse { url } => {
                let transport = rmcp::transport::SseTransport::start(url).await?;
                ().serve(transport).await?
            }
            McpServerTransportConfig::Stdio {
                command,
                args,
                envs,
            } => {
                let transport = rmcp::transport::TokioChildProcess::new(
                    tokio::process::Command::new(command)
                        .args(args)
                        .envs(envs)
                        .stderr(Stdio::null()),
                )?;
                ().serve(transport).await?
            }
        };
        Ok(client)
    }
}

#[derive(Clone, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub config: Option<PathBuf>,
    #[clap(short, long, default_value = "false")]
    pub version: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mcp: McpConfig,
}

impl Config {
    pub fn new(config: Option<PathBuf>) -> Result<Self, ConfigError> {
        let c = FileConfig::builder()
            .add_source(File::from(config.expect("Config file not found")))
            .add_source(Environment::with_prefix("MCP_"))
            .build()?;
        c.try_deserialize()
    }
}
