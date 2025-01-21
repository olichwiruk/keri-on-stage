use ractor::{Actor, ActorProcessingErr, ActorRef};

pub struct WitnessActor;

pub enum WitnessMessage {
    ConfirmEvent,
}

impl Actor for WitnessActor {
    type Msg = WitnessMessage;
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
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WitnessMessage::ConfirmEvent => {
                println!("Witness: Confirmed event.");
            }
        }
        Ok(())
    }
}
