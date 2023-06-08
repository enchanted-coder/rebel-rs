

use anyhow::anyhow;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::StandardFramework, model::gateway::Ready,
    prelude::GatewayIntents,
    
};


use shuttle_secrets::SecretStore;


use songbird::SerenityInit;

use serenity::client::Context;




struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}



#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                    .prefix("~"));
        

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    

    Ok(client.into())
}

