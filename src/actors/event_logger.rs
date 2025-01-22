use super::{
    broker::BrokerMessage,
    ledger::LedgerMessage,
    witness::{WitnessEvent, WitnessMessage},
    SystemMessage,
};
use crate::key::KeyEvent;
use ractor::{registry, Actor, ActorProcessingErr, ActorRef};

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
        let broker = registry::where_is("broker".to_string()).unwrap();
        broker
            .send_message(BrokerMessage::Subscribe(myself))
            .unwrap();
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::EventLogger(msg) = message {
            let broker = registry::where_is("broker".to_string()).unwrap();
            match msg {
                EventLoggerMessage::LogEvent(event) => {
                    let id = &myself.get_id();
                    broker
                        .send_message(BrokerMessage::Publish(
                            SystemMessage::Witness(
                                WitnessMessage::ConfirmEvent(
                                    id.node(),
                                    id.pid(),
                                    event,
                                ),
                            ),
                        ))
                        .unwrap();
                }
            }
        } else if let SystemMessage::WitnessEvent(sys_event) = message {
            let broker = registry::where_is("broker".to_string()).unwrap();
            match sys_event {
                WitnessEvent::EventConfirmed(node, pid, event) => {
                    let id = &myself.get_id();
                    if node != id.node() || pid != id.pid() {
                        return Ok(());
                    }
                    broker
                        .send_message(BrokerMessage::Publish(
                            SystemMessage::Ledger(LedgerMessage::SaveEvent(
                                event,
                            )),
                        ))
                        .unwrap();
                }
            }
        }

        Ok(())
    }
}
