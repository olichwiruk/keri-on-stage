use super::{
    ledger::LedgerMessage,
    witness::{WitnessEvent, WitnessMessage},
    SystemMessage,
};
use crate::key::KeyEvent;
use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

pub struct EventLoggerActor;

#[derive(Debug, Clone)]
pub enum EventLoggerMessage {
    LogEvent(KeyEvent),
}

impl Actor for EventLoggerActor {
    type Msg = SystemMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        pg::join(
            "EventLoggerMessage::LogEvent".to_string(),
            vec![myself.get_cell()],
        );
        pg::join(
            "WitnessEvent::EventConfirmed".to_string(),
            vec![myself.get_cell()],
        );
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::EventLogger(msg) = message {
            match msg {
                EventLoggerMessage::LogEvent(event) => {
                    pg::get_members(
                        &"WitnessMessage::ConfirmEvent".to_string(),
                    )
                    .iter()
                    .for_each(|cell| {
                        cell.send_message(SystemMessage::Witness(
                            WitnessMessage::ConfirmEvent(myself.clone(), event),
                        ))
                        .unwrap();
                    });
                }
            }
        } else if let SystemMessage::WitnessEvent(sys_event) = message {
            match sys_event {
                WitnessEvent::EventConfirmed(event) => {
                    pg::get_members(&"LedgerMessage::SaveEvent".to_string())
                        .iter()
                        .for_each(|cell| {
                            cell.send_message(SystemMessage::Ledger(
                                LedgerMessage::SaveEvent(event),
                            ))
                            .unwrap();
                        });
                }
            }
        }

        Ok(())
    }
}
