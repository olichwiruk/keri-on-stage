use super::{event_logger::EventLoggerMessage, key_manager::KeyManagerMessage};
use crate::key::KeyEventLog;
use ractor::{call, Actor, ActorProcessingErr, ActorRef};

pub struct UserActor;

pub enum UserMessage {
    CreateKey,
}

pub struct UserState {
    kel: KeyEventLog,
    key_manager: ActorRef<KeyManagerMessage>,
    logger: ActorRef<EventLoggerMessage>,
}

impl Actor for UserActor {
    type Msg = UserMessage;
    type State = UserState;
    type Arguments = (ActorRef<KeyManagerMessage>, ActorRef<EventLoggerMessage>);

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (key_manager, logger) = args;

        Ok(Self::State {
            kel: KeyEventLog::new(),
            key_manager,
            logger,
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
                let result =
                    call!(state.key_manager, KeyManagerMessage::CreateEvent)?;
                if let Ok(event) = result {
                    println!("User: Created key: {:?}", event);
                    state.kel.add_event(event);
                    state.logger.cast(EventLoggerMessage::LogEvent(event))?;
                }
            }
        }

        Ok(())
    }
}
