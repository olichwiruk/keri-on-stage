use crate::key::{KeyEvent, KeyEventType};
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

pub struct KeyManagerActor;

pub enum KeyManagerMessage {
    Create(RpcReplyPort<Result<KeyEvent, ()>>),
    Rotate(RpcReplyPort<Result<KeyEvent, ()>>),
}

impl Actor for KeyManagerActor {
    type Msg = KeyManagerMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            KeyManagerMessage::Create(reply) => {
                let event = KeyEvent {
                    event_type: KeyEventType::Inception,
                };
                println!("KeyManager: Created event: {:?}", event);
                reply.send(Ok(event)).unwrap();
            }
            KeyManagerMessage::Rotate(reply) => {
                let event = KeyEvent {
                    event_type: KeyEventType::Rotation,
                };
                println!("KeyManager: Rotated event: {:?}", event);
                reply.send(Ok(event)).unwrap();
            }
        }

        Ok(())
    }
}
