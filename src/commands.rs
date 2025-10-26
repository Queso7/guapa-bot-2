use crate::{BotContext, Data, Error};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct CfUser {
    handle: String,
    rating: Option<i32>,
    maxRating: Option<i32>,
    rank: Option<String>,
}

#[derive(Deserialize)]
struct CfResponse {
    status: String,
    result: Vec<CfUser>,
}

#[poise::command(prefix_command, slash_command, aliases("stalk"))]
pub async fn stalk(
    ctx: BotContext<'_>,
    #[description = "Nombre de usuario"] username: String,
) -> Result<(), Error> {
    let client = Client::new();
    
    let cf_info = match client
        .get(&format!("https://codeforces.com/api/user.info?handles={}", username))
        .send()
        .await
    {
        Ok(resp) => match resp.json::<CfResponse>().await {
            Ok(json) if json.status == "OK" && !json.result.is_empty() => {
                let user = &json.result[0];
                format!(
                    "**Codeforces**: {}\nRating: {}\nMax Rating: {}\nRank: {}",
                    user.handle,
                    user.rating.map_or("N/A".to_string(), |r| r.to_string()),
                    user.maxRating.map_or("N/A".to_string(), |r| r.to_string()),
                    user.rank.clone().unwrap_or("N/A".to_string())
                )
            }
            _ => "No encontrado en Codeforces.".to_string(),
        },
        Err(_) => "Error al consultar Codeforces.".to_string(),
    };

    let ac_info = match client
        .get(&format!("https://atcoder.jp/users/{}", username))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            format!("**AtCoder**: Perfil encontrado: https://atcoder.jp/users/{}", username)
        }
        _ => "No encontrado en AtCoder.".to_string(),
    };

    ctx.say(format!("{}\n{}", cf_info, ac_info)).await?;

    Ok(())
}


#[poise::command(prefix_command, slash_command, aliases("p"))]
pub async fn ping(ctx: BotContext<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, aliases("hi", "hola"))]
pub async fn hello(
    ctx: BotContext<'_>,
    #[description = "Nombre de la persona"] name: Option<String>,
) -> Result<(), Error> {
    let name = name.unwrap_or_else(|| ctx.author().name.clone());
    ctx.say(format!("Hello, {}!", name)).await?;
    Ok(())
}

pub fn register_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping(),
        hello(),
        stalk(),
    ]
}
