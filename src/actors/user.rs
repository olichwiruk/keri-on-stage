use super::{
    event_logger::EventLoggerMessage,
    key_manager::{KeyManagerEvent, KeyManagerMessage},
    SystemMessage,
};
use crate::key::KeyEventLog;
use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

pub struct UserActor;

#[derive(Debug, Clone)]
pub enum UserMessage {
    CreateKey,
    RotateKey,
}

pub struct UserState {
    kel: KeyEventLog,
}

impl Actor for UserActor {
    type Msg = SystemMessage;
    type State = UserState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(Self::State {
            kel: KeyEventLog::new(),
        })
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::User(msg) = message {
            match msg {
                UserMessage::CreateKey => {
                    pg::get_members(&"KeyManagerMessage::Create".to_string())
                        .iter()
                        .for_each(|cell| {
                            cell.send_message(SystemMessage::KeyManager(
                                KeyManagerMessage::Create(myself.clone()),
                            )).unwrap();
                        });
                }
                UserMessage::RotateKey => {
                    pg::get_members(&"KeyManagerMessage::Rotate".to_string())
                        .iter()
                        .for_each(|cell| {
                            cell.send_message(SystemMessage::KeyManager(
                                KeyManagerMessage::Rotate(myself.clone()),
                            )).unwrap();
                        });
                }
            }
        } else if let SystemMessage::KeyManagerEvent(sys_event) = message {
            match sys_event {
                KeyManagerEvent::Created(event) 
                | KeyManagerEvent::Rotated(event) => {
                    println!("User {} received event: {:?}", myself.get_id(), event);
                    state.kel.add_event(event);
                    pg::get_members(&"EventLoggerMessage::LogEvent".to_string())
                        .iter()
                        .for_each(|cell| {
                            cell.send_message(SystemMessage::EventLogger(
                                EventLoggerMessage::LogEvent(event),
                            )).unwrap();
                        });
                }
            }
        }

        Ok(())
    }
}
