use ractor::Actor;

mod actors;
mod key;
use actors::{
    broker::BrokerActor,
    event_logger::EventLoggerActor,
    key_manager::KeyManagerActor,
    ledger::LedgerActor,
    user::{UserActor, UserMessage},
    witness::WitnessActor,
    SystemMessage,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Actor::spawn(Some("broker".to_string()), BrokerActor, ()).await?;

    let _ = Actor::spawn(None, WitnessActor, ()).await?;
    let _ = Actor::spawn(None, LedgerActor, ()).await?;
    let _ = Actor::spawn(None, EventLoggerActor, ()).await?;
    let _ = Actor::spawn(None, KeyManagerActor, ()).await?;

    let (user_actor, user_handle) = Actor::spawn(None, UserActor, ()).await?;
    user_actor.cast(SystemMessage::User(UserMessage::CreateKey))?;
    user_actor.cast(SystemMessage::User(UserMessage::RotateKey))?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    user_actor.stop(None);
    user_handle.await?;

    Ok(())
}
