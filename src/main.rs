use ractor::Actor;

mod actors;
use actors::{
    event_logger::EventLoggerActor,
    key_manager::{KeyManagerActor, KeyManagerMessage},
    witness::WitnessActor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Actor::spawn(Some("witness".to_string()), WitnessActor, ()).await?;
    let (logger_actor, _) = Actor::spawn(None, EventLoggerActor, ()).await?;

    let (key_manager_actor, key_manager_handle) =
        Actor::spawn(None, KeyManagerActor, logger_actor).await?;

    key_manager_actor.cast(KeyManagerMessage::CreateEvent)?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    key_manager_actor.stop(None);
    key_manager_handle.await?;

    Ok(())
}
