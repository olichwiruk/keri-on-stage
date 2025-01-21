use ractor::Actor;

mod actors;
mod key;
use actors::{
    event_logger::EventLoggerActor,
    key_manager::KeyManagerActor,
    ledger::LedgerActor,
    user::{UserActor, UserMessage},
    witness::WitnessActor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Actor::spawn(None, WitnessActor, ()).await?;
    let _ = Actor::spawn(None, LedgerActor, ()).await?;
    let (logger_actor, _) = Actor::spawn(None, EventLoggerActor, ()).await?;
    let (key_manager_actor, _) =
        Actor::spawn(None, KeyManagerActor, ()).await?;

    let (user_actor, user_handle) =
        Actor::spawn(None, UserActor, (key_manager_actor, logger_actor))
            .await?;

    user_actor.cast(UserMessage::CreateKey)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    user_actor.stop(None);
    user_handle.await?;

    Ok(())
}
