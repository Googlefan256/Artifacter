use localization::t;
use poise::serenity_prelude as serenity;
use poise::CreateReply;

use crate::db::find_hsr;
use crate::hsr_components::hsr_profile_components;
use crate::state::{Context, Error};
use serenity::User;

/// get user's profile
#[poise::command(
    context_menu_command = "Get HSR Info",
    description_localized("ja", "プロフィールを表示します"),
    slash_command
)]
pub async fn hsr_profile(ctx: Context<'_>, user: User) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let data = ctx.data();
    let uid = if let Ok(Some(db_uid)) = find_hsr(&data.db, user.id.get()).await {
        db_uid
    } else {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.noUserId")))
            .await?;
        return Ok(());
    };
    if uid.to_string().len() != 9 {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.userIdMustBeNineDigits")))
            .await?;
        return Ok(());
    }
    ctx.defer().await?;
    let user = match data
        .api
        .hsr_profile(uid.clone(), Some(locale.to_string()))
        .await
    {
        Ok(user) => user,
        Err(_) => {
            ctx.send(CreateReply::new().content(t!(locale, "main:general.failedToFetchData")))
                .await?;
            return Ok(());
        }
    };
    if user.characters.is_empty() {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.noCharacters")))
            .await?;
        return Ok(());
    }
    let (embed, components, attachment) = hsr_profile_components(locale.to_string(), uid, user);
    let mut builder = CreateReply::default().components(components).embed(embed);
    if let Some(attachment) = attachment {
        builder = builder.attachment(attachment);
    }
    ctx.send(builder).await?;
    Ok(())
}
