use ractor::{registry, Actor, ActorProcessingErr, ActorRef};

use crate::key::KeyEvent;

use super::{broker::BrokerMessage, SystemMessage};

pub struct WitnessActor;

#[derive(Debug, Clone)]
pub enum WitnessMessage {
    ConfirmEvent(u64, u64, KeyEvent),
}

#[derive(Debug, Clone)]
pub enum WitnessEvent {
    EventConfirmed(u64, u64, KeyEvent),
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
        let broker = registry::where_is("broker".to_string()).unwrap();
        broker
            .send_message(BrokerMessage::Subscribe(myself))
            .unwrap();
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::Witness(msg) = message {
            let broker = registry::where_is("broker".to_string()).unwrap();
            match msg {
                WitnessMessage::ConfirmEvent(node, pid, event) => broker
                    .send_message(BrokerMessage::Publish(
                        SystemMessage::WitnessEvent(
                            WitnessEvent::EventConfirmed(node, pid, event),
                        ),
                    ))?,
            }
        }
        Ok(())
    }
}
