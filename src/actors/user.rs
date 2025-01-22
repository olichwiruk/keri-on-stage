use super::{event_logger::EventLoggerMessage, key_manager::KeyManagerMessage};
use crate::key::{KeyEvent, KeyEventLog};
use ractor::{
    call, registry, Actor, ActorProcessingErr, ActorRef, RpcReplyPort,
};

pub struct UserActor;

pub enum UserMessage {
    CreateKey,
    RotateKey,
    ListEvents(RpcReplyPort<Vec<KeyEvent>>),
}

pub struct UserState {
    kel: KeyEventLog,
}

impl Actor for UserActor {
    type Msg = UserMessage;
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
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            UserMessage::CreateKey => {
                let key_manager: ActorRef<KeyManagerMessage> =
                    registry::where_is("key_manager".to_string())
                        .unwrap()
                        .into();
                let logger: ActorRef<EventLoggerMessage> =
                    registry::where_is("logger".to_string()).unwrap().into();

                let result = call!(key_manager, KeyManagerMessage::Create)?;
                if let Ok(event) = result {
                    println!("User: Created key: {:?}", event);
                    state.kel.add_event(event);
                    logger.cast(EventLoggerMessage::LogEvent(event))?;
                }
            }
            UserMessage::RotateKey => {
                let key_manager: ActorRef<KeyManagerMessage> =
                    registry::where_is("key_manager".to_string())
                        .unwrap()
                        .into();
                let logger: ActorRef<EventLoggerMessage> =
                    registry::where_is("logger".to_string()).unwrap().into();

                let result = call!(key_manager, KeyManagerMessage::Rotate)?;
                if let Ok(event) = result {
                    println!("User: Created key: {:?}", event);
                    state.kel.add_event(event);
                    logger.cast(EventLoggerMessage::LogEvent(event))?;
                }
            }
            UserMessage::ListEvents(reply) => {
                reply.send(state.kel.get_events()).unwrap();
            }
        }

        Ok(())
    }
}
