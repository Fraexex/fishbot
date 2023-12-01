use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const HELP_MESSAGE: &str = "
Hello! Welcome to Nova CS club! Let's get you started:

❓ Rules?
-> See <#CHANNEL_ID> for information as well as Discord TOS. If you have any
issues with any of the rules, bring it up with @ibaad_18

❓ Need help?
-> Post in the <#CHANNEL_ID> channel to get assistance from other members.

❓ Suggestions for future projects?
-> Post in the <#CHANNEL_ID> channel to post your suggestions.
";

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

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

/// Responds with helpful information
#[poise::command(slash_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(HELP_MESSAGE).await?;
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
                help()
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
