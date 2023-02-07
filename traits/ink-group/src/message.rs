#[ink::trait_definition]
pub trait Dao {
    #[ink(message)]
    fn get(&self) -> bool;
}
