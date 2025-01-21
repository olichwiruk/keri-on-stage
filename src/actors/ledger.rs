use crate::key::KeyEvent;
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub struct LedgerActor;

pub enum LedgerMessage {
    SaveEvent(KeyEvent),
}

impl Actor for LedgerActor {
    type Msg = LedgerMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            LedgerMessage::SaveEvent(event) => {
                println!("Ledger: saved event: {:?}", event);
            }
        }
        Ok(())
    }
}
