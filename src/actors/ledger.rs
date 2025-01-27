use crate::key::KeyEvent;
use ractor::{pg, Actor, ActorProcessingErr, ActorRef};

use super::SystemMessage;

pub struct LedgerActor<P: Parser> {
    pub _phantom: std::marker::PhantomData<P>,
}
impl<P: Parser> LedgerActor<P> {
    pub fn new() -> LedgerActor<P> {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LedgerMessage {
    SaveEvent(KeyEvent),
}

pub trait Parser {
    fn parse(&self, event: KeyEvent) -> String;
}
pub struct JsonParser;
pub struct PlainParser;

impl Parser for JsonParser {
    fn parse(&self, event: KeyEvent) -> String {
        format!("{{\"event\": \"{:?}\"}}", event)
    }
}
impl Parser for PlainParser {
    fn parse(&self, event: KeyEvent) -> String {
        format!("{:?}", event)
    }
}

pub struct LedgerState<P: Parser> {
    parser: P,
}

impl<P: Parser + Send + Sync + 'static> Actor for LedgerActor<P> {
    type Msg = SystemMessage;
    type State = LedgerState<P>;
    type Arguments = P;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        parser: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        pg::join("LedgerMessage::SaveEvent".to_string(), vec![myself.get_cell()]);
        Ok(LedgerState { parser })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SystemMessage::Ledger(msg) = message {
            match msg {
                LedgerMessage::SaveEvent(event) => {
                    let parsed = state.parser.parse(event);
                    println!("Ledger: saved event: {parsed:}");
                }
            }
        }
        Ok(())
    }
}
