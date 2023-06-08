
mod commands;

use std::time::Duration;
use tokio::time::sleep;
use std::collections::{HashMap, HashSet};
use anyhow::anyhow;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler}, model::prelude::{GuildId}
};

use serenity::model::application::interaction::{InteractionResponseType, Interaction};

use shuttle_secrets::SecretStore;
use songbird::SerenityInit;

use core::panic;


use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;


use serenity::framework::standard::StandardFramework;
use serenity::http::Http;


use serenity::model::gateway::Ready;



use serenity::prelude::*;
use tracing::error;




pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
       

    
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            println!("{} is connected on shard {}/{}", ready.user.name, shard[0], shard[1]);
        }

        let guild_id = GuildId(746365930224746557);

       let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
            .create_application_command(|command| commands::ping::register(command))
            
       })
       .await;

       println!("I now have the following guild slash commands: {:#?}", commands);

      
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

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                    .with_whitespace(true)
                    .on_mention(Some(bot_id))
                    .prefix("~")
                    // In this case, if "," would be first, a message would never
                   // be delimited at ", ", forcing you to trim your arguments if you
                   // want to avoid whitespaces at the start of each.
                   .delimiters(vec![", ", ", "])
                   // Sets the bot's owners. These will be used for commands that
                   // are owners only.
                   .owners(owners));
    
        



        

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop{
            sleep(Duration::from_secs(30)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });

    // starts shards

    if let Err(why) = client.start_shards(2).await {
        error!("Client error: {:?}", why);
    }

    Ok(client.into())

    

    
}