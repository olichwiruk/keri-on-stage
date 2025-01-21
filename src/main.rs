use ractor::{Actor, ActorRef, ActorProcessingErr};

#[tokio::main]
async fn main() {
    let (actor, actor_handle) = Actor::spawn(None, MyFirstActor, ()).await.expect("Actor failed to start");
    
    for _i in 0..10 {
        actor.cast(MyFirstActorMessage::PrintHelloWorld).expect("Failed to send message to actor");
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    actor.stop(None);
    actor_handle.await.unwrap();
}

pub struct MyFirstActor;

pub enum MyFirstActorMessage {
    PrintHelloWorld,
}

impl Actor for MyFirstActor {
    type State = ();
    type Msg = MyFirstActorMessage;
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _arguments: Self::Arguments)
        -> Result<Self::State, ActorProcessingErr>
    {
        Ok(())
    }

    async fn handle(&self, _myself: ActorRef<Self::Msg>, message: Self::Msg, _state: &mut Self::State) 
        -> Result<(), ActorProcessingErr>
    {
        match message {
            MyFirstActorMessage::PrintHelloWorld => {
                println!("Hello world!");
            }
        }
        Ok(())
    }
}
