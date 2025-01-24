use crate::key::{KeyEvent, KeyEventType};
use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

use super::SystemMessage;

pub struct KeyManagerActor;

#[derive(Debug, Clone)]
pub enum KeyManagerMessage {
    Create(ActorRef<SystemMessage>),
    Rotate(ActorRef<SystemMessage>),
}

#[derive(Debug, Clone)]
pub enum KeyManagerEvent {
    Created(KeyEvent),
    Rotated(KeyEvent),
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
        pg::join("KeyManagerMessage::Create".to_string(), vec![myself.get_cell()]);
        pg::join("KeyManagerMessage::Rotate".to_string(), vec![myself.get_cell()]);

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
                KeyManagerMessage::Create(sender) => {
                    let event = KeyEvent {
                        event_type: KeyEventType::Inception,
                    };
                    sender.cast(
                        SystemMessage::KeyManagerEvent(
                            KeyManagerEvent::Created(event),
                        ),
                    )?
                }
                KeyManagerMessage::Rotate(sender) => {
                    let event = KeyEvent {
                        event_type: KeyEventType::Rotation,
                    };
                    sender.cast(
                        SystemMessage::KeyManagerEvent(
                            KeyManagerEvent::Rotated(event),
                        ),
                    )?
                }
            }
        }

        Ok(())
    }
}
