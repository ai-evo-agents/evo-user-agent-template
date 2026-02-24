use async_trait::async_trait;
use evo_agent_sdk::prelude::*;

struct MyAgent;

#[async_trait]
impl AgentHandler for MyAgent {
    async fn on_pipeline(&self, ctx: PipelineContext<'_>) -> anyhow::Result<serde_json::Value> {
        // TODO: Implement your agent logic here.
        //
        // Use ctx.gateway to call LLMs via evo-gateway:
        let response = ctx
            .gateway
            .chat_completion("gpt-4o-mini", &ctx.soul.behavior, "Hello", None, None)
            .await?;

        Ok(serde_json::json!({ "result": response }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AgentRunner::run(MyAgent).await
}
