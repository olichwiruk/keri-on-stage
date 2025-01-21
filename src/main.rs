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
    let _ =
        Actor::spawn(Some("logger".to_string()), EventLoggerActor, ()).await?;
    let _ = Actor::spawn(Some("key_manager".to_string()), KeyManagerActor, ())
        .await?;

    let (user_actor, user_handle) = Actor::spawn(None, UserActor, ()).await?;
    user_actor.cast(UserMessage::CreateKey)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    user_actor.stop(None);
    user_handle.await?;

    Ok(())
}
