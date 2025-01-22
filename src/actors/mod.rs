pub mod broker;
pub mod event_logger;
pub mod key_manager;
pub mod ledger;
pub mod user;
pub mod witness;

#[derive(Debug, Clone)]
pub enum SystemMessage {
    EventLogger(event_logger::EventLoggerMessage),
    KeyManager(key_manager::KeyManagerMessage),
    KeyManagerEvent(key_manager::KeyManagerEvent),
    Ledger(ledger::LedgerMessage),
    User(user::UserMessage),
    Witness(witness::WitnessMessage),
    WitnessEvent(witness::WitnessEvent),
}
