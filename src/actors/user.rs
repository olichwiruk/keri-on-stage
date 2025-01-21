use super::key_manager::KeyManagerMessage;
use crate::key::KeyEventLog;
use ractor::{call, Actor, ActorProcessingErr, ActorRef};

pub struct UserActor;

pub enum UserMessage {
    CreateKey,
}

pub struct UserState {
    kel: KeyEventLog,
    key_manager: ActorRef<KeyManagerMessage>,
}

impl Actor for UserActor {
    type Msg = UserMessage;
    type State = UserState;
    type Arguments = ActorRef<KeyManagerMessage>;

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(Self::State {
            kel: KeyEventLog::new(),
            key_manager: args,
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
                }
            }
        }

        Ok(())
    }
}
