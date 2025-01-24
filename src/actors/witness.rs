use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

use crate::key::KeyEvent;

use super::SystemMessage;

pub struct WitnessActor;

#[derive(Debug, Clone)]
pub enum WitnessMessage {
    ConfirmEvent(ActorRef<SystemMessage>, KeyEvent),
}

#[derive(Debug, Clone)]
pub enum WitnessEvent {
    EventConfirmed(KeyEvent),
}

impl Actor for WitnessActor {
    type Msg = SystemMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        pg::join(
            "WitnessMessage::ConfirmEvent".to_string(),
            vec![myself.get_cell()],
        );
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::Witness(msg) = message {
            match msg {
                WitnessMessage::ConfirmEvent(sender, event) => {
                    sender.cast(SystemMessage::WitnessEvent(
                        WitnessEvent::EventConfirmed(event),
                    ))?
                }
            }
        }
        Ok(())
    }
}
