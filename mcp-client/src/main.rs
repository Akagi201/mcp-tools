mod config;
mod log;

use clap::Parser;
use rmcp::{model::CallToolRequestParam, object};
use shadow_rs::shadow;
use tracing::info;

use crate::{
    config::{Cli, Config},
    log::init_log,
};

shadow!(build);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init_log("info")?;
    let cli = Cli::parse();
    if cli.version {
        println!("{}", build::VERSION);
        return Ok(());
    }

    let config = Config::new(cli.config)?;
    info!("{:?}", config);

    let mcp_client = config.mcp.create_client().await?;
    info!("MCP client created: {:?}", mcp_client);

    let server_info = mcp_client.peer_info();
    info!("Server info: {:?}", server_info);

    let tools = mcp_client.list_tools(Default::default()).await?;
    info!("Available tools: {:#?}", tools);

    let tool_result = mcp_client
        .call_tool(CallToolRequestParam {
            name: "get_pairs_by_token_addresses".into(),
            arguments: Some(object!({ "token_addresses": ["0xbae207659db88bea0cbead6da0ed00aac12edcdda169e591cd41c94180b46f3b"] })),
        })
        .await?;
    info!("Tool result: {tool_result:#?}");

    mcp_client.cancel().await?;
    Ok(())
}
