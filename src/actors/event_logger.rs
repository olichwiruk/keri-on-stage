use super::{ledger::LedgerMessage, witness::WitnessMessage};
use crate::key::KeyEvent;
use ractor::{call, pg, Actor, ActorProcessingErr, ActorRef};

pub struct EventLoggerActor;

pub enum EventLoggerMessage {
    LogEvent(KeyEvent),
}

impl Actor for EventLoggerActor {
    type Msg = EventLoggerMessage;
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
            EventLoggerMessage::LogEvent(event) => {
                println!("EventLogger: Logged event: {:?}", event);
                let witnesses = pg::get_members(&"witnesses".to_string());
                let witness: ActorRef<WitnessMessage> =
                    witnesses.first().unwrap().clone().into();

                let ledgers: Vec<ActorRef<LedgerMessage>> =
                    pg::get_members(&"ledgers".to_string())
                        .iter_mut()
                        .map(|l| l.clone().into())
                        .collect();

                let result = call!(witness, WitnessMessage::ConfirmEvent)?;
                if let Ok(()) = result {
                    println!("EventLogger: Witness confirmed event.");
                    ledgers.iter().for_each(|ledger| {
                        ledger.cast(LedgerMessage::SaveEvent(event)).unwrap();
                    });
                }
            }
        }

        Ok(())
    }
}
