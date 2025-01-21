use super::event_logger::EventLoggerMessage;
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub struct KeyManagerActor;

pub enum KeyManagerMessage {
    CreateEvent,
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
            KeyManagerMessage::CreateEvent => {
                let event = "Inception Event".to_string();
                println!("KeyManager: Created event: {}", event);
                state.logger.cast(EventLoggerMessage::LogEvent(event))?;
            }
        }

        Ok(())
    }
}
