use super::event_logger::EventLoggerMessage;
use crate::key::{KeyEvent, KeyEventType};
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

pub struct KeyManagerActor;

pub enum KeyManagerMessage {
    CreateEvent(RpcReplyPort<Result<KeyEvent, ()>>),
}

pub struct KeyManagerState {
    logger: ActorRef<EventLoggerMessage>,
}

impl Actor for KeyManagerActor {
    type Msg = KeyManagerMessage;
    type State = KeyManagerState;
    type Arguments = ActorRef<EventLoggerMessage>;

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(Self::State { logger: args })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            KeyManagerMessage::CreateEvent(reply) => {
                let event = KeyEvent {
                    event_type: KeyEventType::Inception,
                };
                println!("KeyManager: Created event: {:?}", event);
                state.logger.cast(EventLoggerMessage::LogEvent(event))?;
                reply.send(Ok(event)).unwrap();
            }
        }

        Ok(())
    }
}
