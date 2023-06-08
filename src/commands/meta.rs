use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};
use std::fmt::Write;

use crate::CommandCounter;

#[command]
#[bucket = "complicated"]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter {
        writeln!(contents, "- {name}: {amount}", name = k, amount = v)?;
    }

    msg.channel_id.say(&ctx.http, &contents).await?;

    Ok(())
}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    match args.single_quoted::<String>() {
        Ok(x) => {
            let settings = if let Some(guild_id) = msg.guild_id {
                // by default roles, users and channel mentions are cleaned.
                ContentSafeOptions::default()
                    // we dont clean channel mentions
                    // as they do not ping users.
                    .clean_channel(false)
                    // to display users as their display names in guilds.
                    .display_as_member_from(guild_id)
            } else {
                ContentSafeOptions::default().clean_channel(false).clean_role(false)
            };

            let content = content_safe(&ctx.cache, x, &settings, &msg.mentions);

            msg.channel_id.say(&ctx.http, &content).await?;

            return Ok(());
        },
        Err(_) => {
            msg.reply(ctx, "An argument is required to run this command.").await?;
            return Ok(());
        },
    };
}

