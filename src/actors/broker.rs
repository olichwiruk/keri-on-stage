use super::SystemMessage;
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub struct BrokerActor;

pub enum BrokerMessage {
    Subscribe(ActorRef<SystemMessage>),
    Publish(SystemMessage),
}

pub struct BrokerState {
    subscribers: Vec<ActorRef<SystemMessage>>,
}

impl Actor for BrokerActor {
    type Msg = BrokerMessage;
    type State = BrokerState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(Self::State {
            subscribers: vec![],
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            BrokerMessage::Subscribe(actor) => {
                state.subscribers.push(actor);
            }
            BrokerMessage::Publish(msg) => {
                state.subscribers.iter().for_each(|sub| {
                    let _ = sub.cast(msg.clone());
                });
            }
        }

        Ok(())
    }
}
