use crate::key::KeyEvent;
use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

use super::SystemMessage;

pub struct LedgerActor;

#[derive(Debug, Clone)]
pub enum LedgerMessage {
    SaveEvent(KeyEvent),
}

impl Actor for LedgerActor {
    type Msg = SystemMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        pg::join("LedgerMessage::SaveEvent".to_string(), vec![myself.get_cell()]);
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::Ledger(msg) = message {
            match msg {
                LedgerMessage::SaveEvent(event) => {
                    println!("Ledger: saved event: {:?}", event);
                }
            }
        }
        Ok(())
    }
}
