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
            /2plus2 - Extensive proof that 2 + 2 is indeed fish!\n\n
            
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

/// 2 + 2 = Fish Proof
#[poise::command(slash_command)]
async fn fishproof(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("In this first proof, we'll define what exactly it means to add something. Verb: Put in (an additional element, ingredient, etc).
            So if the addition symbol (+) is symbolizing this meaning of add, then it wouldn't be unruly to suggest that
            2 + 2 equals fish. In this example instance, add will describe the act of merging; to have something within a something.
            This act of adding cares not about the orientation of the object, as it simply means to have it put in. Thus, 2 can
            be orientated like this '2' and once placed upon it, it bears resemblance to that of 🐟.\n\n
            
            For this second proof, we will decompose 2 & 2 into the smaller components. This would yield in the fact that 2 is composed of 1 + 1,
            thus, 2 + 2 = 1 + 1 + 1 + 1. All numbers are a social construct, meant to represent some object/idea. So lets let 1 equal to some char.
            As of Unicode version 15.1, there are 149,878 characters. Because 'fish' is a four letter word, this'd mean that there'd need to be
            149,878^4 random iterations, until by chance, f + i + s + h is achieved. Here, the random propabiilty theory proves that 2 + 2 = fish
            through its clever use of the laws of entropy."
        ).await?;
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
                age(),
                help(),
                fishproof(),
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
