use ractor::{pg, Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

pub struct WitnessActor;

pub enum WitnessMessage {
    ConfirmEvent(RpcReplyPort<Result<(), ()>>),
}

impl Actor for WitnessActor {
    type Msg = WitnessMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        pg::join("witnesses".to_string(), vec![myself.get_cell()]);
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WitnessMessage::ConfirmEvent(replay) => {
                println!("Witness: Confirmed event.");
                replay.send(Ok(())).unwrap();
            }
        }
        Ok(())
    }
}
