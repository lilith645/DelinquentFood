pub use self::fridge::Fridge;
pub use self::dishwasher::Dishwasher;
pub use self::meat_tenderizer::MeatTenderizer;
pub use self::coffee_machine::CoffeeMachine;

pub mod traits;
mod fridge;
mod dishwasher;
mod meat_tenderizer;
mod coffee_machine;
