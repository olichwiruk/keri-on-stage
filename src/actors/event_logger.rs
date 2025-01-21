use super::{ledger::LedgerMessage, witness::WitnessMessage};
use ractor::{call, Actor, ActorProcessingErr, ActorRef};

pub struct EventLoggerActor;

pub enum EventLoggerMessage {
    LogEvent(String),
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
                println!("EventLogger: Logged event: {}", event);
                let witness: ActorRef<WitnessMessage> =
                    ractor::registry::where_is("witness".to_string())
                        .unwrap()
                        .into();
                let ledger: ActorRef<LedgerMessage> =
                    ractor::registry::where_is("ledger".to_string())
                        .unwrap()
                        .into();

                let result = call!(witness, WitnessMessage::ConfirmEvent)?;
                if let Ok(()) = result {
                    println!("EventLogger: Witness confirmed event.");
                    ledger.cast(LedgerMessage::SaveEvent(event))?;
                }
            }
        }

        Ok(())
    }
}
