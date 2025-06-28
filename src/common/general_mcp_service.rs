use std::sync::Arc;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, schemars,
    service::RequestContext, tool,
};
use serde_json::json;
use tokio::sync::Mutex;



#[derive(Clone)]
pub struct GeneralMcpService {
    
}

#[tool(tool_box)]
impl GeneralMcpService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            
        }
    }



    #[tool(description = "Get the current weather in a given location")]
    async fn get_current_weather(
        &self,
        #[tool(param)]
        #[schemars(description = "Location for which you desire to know weather. You can specify an Unit like Degree Celsius or Degree Farenheit")]
        location: String,
        #[tool(param)]
        #[schemars(description = "Temperature unit to use")]
        unit: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        let unit = unit.unwrap_or("Degree Celsius".to_string());
        let begining_string=r#""{"Temperature": "24", "unit":""#;
        let end_string=r#"","description":"Sunny"}"#;
        Ok(CallToolResult::success(vec![Content::text(
            format!("{}{}{}",begining_string,unit,end_string),

        )]))
    }

    #[tool(description = "Give customer details")]
    async fn get_customer_details(
        &self,
        #[tool(param)]
        #[schemars(description = "Give customer details from a given customer_id")]
        customer_id: String,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            r#"{"Full Name": "Company A", "address": "Sunny Street"}"#,
        )]))
    }



}

const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for GeneralMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides  a function 'get_current_weather' to retrieve weather from a specific location,  'get_customer_details' to get info about a customer".to_string()),
        }
    }

  
}