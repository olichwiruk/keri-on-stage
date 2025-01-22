use crate::key::{KeyEvent, KeyEventType};
use ractor::{registry, Actor, ActorProcessingErr, ActorRef};

use super::{broker::BrokerMessage, SystemMessage};

pub struct KeyManagerActor;

#[derive(Debug, Clone)]
pub enum KeyManagerMessage {
    Create(u64, u64),
    Rotate(u64, u64),
}

#[derive(Debug, Clone)]
pub enum KeyManagerEvent {
    Created(u64, u64, KeyEvent),
    Rotated(u64, u64, KeyEvent),
}

impl Actor for KeyManagerActor {
    type Msg = SystemMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let broker = registry::where_is("broker".to_string()).unwrap();
        broker.send_message(BrokerMessage::Subscribe(myself))?;

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::KeyManager(msg) = message {
            match msg {
                KeyManagerMessage::Create(node, pid) => {
                    let event = KeyEvent {
                        event_type: KeyEventType::Inception,
                    };
                    let broker =
                        registry::where_is("broker".to_string()).unwrap();
                    broker.send_message(BrokerMessage::Publish(
                        SystemMessage::KeyManagerEvent(
                            KeyManagerEvent::Created(node, pid, event),
                        ),
                    ))?
                }
                KeyManagerMessage::Rotate(node, pid) => {
                    let event = KeyEvent {
                        event_type: KeyEventType::Rotation,
                    };
                    let broker =
                        registry::where_is("broker".to_string()).unwrap();
                    broker.send_message(BrokerMessage::Publish(
                        SystemMessage::KeyManagerEvent(
                            KeyManagerEvent::Rotated(node, pid, event),
                        ),
                    ))?
                }
            }
        }

        Ok(())
    }
}
