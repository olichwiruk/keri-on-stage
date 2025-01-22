use super::{
    event_logger::EventLoggerMessage,
    key_manager::{KeyManagerEvent, KeyManagerMessage},
    SystemMessage,
};
use crate::{actors::broker::BrokerMessage, key::KeyEventLog};
use ractor::{registry, Actor, ActorProcessingErr, ActorRef};

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
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let broker = registry::where_is("broker".to_string()).unwrap();
        broker.send_message(BrokerMessage::Subscribe(myself))?;

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
            let broker: ActorRef<BrokerMessage> =
                registry::where_is("broker".to_string()).unwrap().into();

            match msg {
                UserMessage::CreateKey => {
                    let id = &myself.get_id();
                    broker.cast(BrokerMessage::Publish(
                        SystemMessage::KeyManager(KeyManagerMessage::Create(
                            id.node(),
                            id.pid(),
                        )),
                    ))?;
                }
                UserMessage::RotateKey => {
                    let id = &myself.get_id();
                    broker.cast(BrokerMessage::Publish(
                        SystemMessage::KeyManager(KeyManagerMessage::Rotate(
                            id.node(),
                            id.pid(),
                        )),
                    ))?;
                }
            }
        } else if let SystemMessage::KeyManagerEvent(sys_event) = message {
            let broker: ActorRef<BrokerMessage> =
                registry::where_is("broker".to_string()).unwrap().into();
            match sys_event {
                KeyManagerEvent::Created(node, pid, event)
                | KeyManagerEvent::Rotated(node, pid, event) => {
                    let id = &myself.get_id();
                    if node != id.node() || pid != id.pid() {
                        return Ok(());
                    }
                    state.kel.add_event(event);
                    broker.cast(BrokerMessage::Publish(
                        SystemMessage::EventLogger(
                            EventLoggerMessage::LogEvent(event),
                        ),
                    ))?;

                    println!("User {} kel: {:#?}", id, state.kel.events);
                }
            }
        }

        Ok(())
    }
}
