use poise::{serenity_prelude as serenity, FrameworkContext};
use serenity::FullEvent;

use crate::state::{Error, State};
mod ready;

type Framework<'a> = FrameworkContext<'a, State, Error>;

pub async fn handler(event: &FullEvent, frame: Framework<'_>, state: &State) -> Result<(), Error> {
    match event {
        FullEvent::Ready {
            ctx,
            data_about_bot,
        } => {
            ready::handler(frame, state, ctx, data_about_bot).await?;
        }
        _ => {}
    }
    Ok(())
}
