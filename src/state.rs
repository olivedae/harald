use std::fmt::{Debug, Formatter, Result};
use std::clone::Clone;

pub enum State {
    Unknown,
    Unsupported,
    Unauthorized,
    PoweredOff,
    PoweredOn,
}

impl State {
    fn id(&self) -> usize {
        match *self {
            State::Unknown      => 1,
            State::Unsupported  => 3,
            State::Unauthorized => 4,
            State::PoweredOff   => 5,
            State::PoweredOn    => 6,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.id() == other.id()
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "State::{}", match *self {
            State::Unknown      => "Unknown",
            State::Unsupported  => "Unsupported",
            State::Unauthorized => "Unauthorized",
            State::PoweredOff   => "PoweredOff",
            State::PoweredOn    => "PoweredOn",
        })
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        match *self {
            State::Unknown      => State::Unknown,
            State::Unsupported  => State::Unsupported,
            State::Unauthorized => State::Unauthorized,
            State::PoweredOff   => State::PoweredOff,
            State::PoweredOn    => State::PoweredOn,
        }
    }
}
