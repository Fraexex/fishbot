use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;
use serenity::utils::Colour;
use poise::serenity_prelude::ButtonStyle;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Display user account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>)
-> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Respond with helpful information
#[poise::command(slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| b.description(
            "Fishbot is designed to support all your fishy needs! Here are the supported commands:\n\n
            /help - Receive a copy of this message.\n\n
            /fish - Get a random fish image!\n\n
            /anifish - Get a random anime-style fish image!\n\n
            /jankenpon - Play rock-paper-scissors FISH!\n\n
            /gofish - The popular match-two card game...\n\n
            
            ❓ Need help?\n
            -> Post in <#CHANNEL_ID> to get assistance from other members.\n\n
            
            ❓ Suggestions?\n
            -> Post in <#CHANNEL_ID> to post your suggestions."
        ).title("Help").colour(Colour::BLITZ_BLUE))
        .ephemeral(true)
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| {
                    b.label("Support information")
                        .url("https://discord.gg/bRnvhSBV5c")
                        .style(ButtonStyle::Link)
                })
            })
        })
    })
    .await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Add commands here
                hello(),
                age(),
                help(),
            ],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
