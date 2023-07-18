use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


/// A state machine - Generic over the transition type
pub trait StateMachine {
    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;
}



// Simple helper to do some hashing.
fn hash<T>(t: &T) -> u64 {
    todo!("Final Project");
}

// Test for hash function 
#[test]
fn test_hash_enum_vec() {
    enum KeyTest{
        One,
        Two,
        Three,
        Four

    }
    let input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];

    let hash1 = hash(&input);
    let hash2 = hash(&input);

    assert_eq!(hash1, hash2);
}
