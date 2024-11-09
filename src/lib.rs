//! 
//! pedal-board - Audio effect software pedal board.
//! 
//!
//! provides a library of rust elements that implement an abstracted version of a pedal board with a number of 
//! software implementations of common effect pedals.
//!
//! A PedalBoard is a container that holds a collection of Pedals.  The Pedal is an interface that all implementations
//! adhere to.  Pedals are chained together and their effects are called in sequence to modify buffers of audio
//! data.
//! 
//! Pedals are built using dsp functions that provide building blocks.  dsp functions are provided for a biquad filter,
//! attack_hold_release, clip functions, etc.  There is also a pedal that models a Princeton amp that was built
//! using Faust and then exported to rust and ported to fit the Pedal interface (not much work).
//! 
//! All pedal representation is done using json representation.  One of the required functions the Pedal trait
//! enforces is an as_json() that the specific pedal implements to describe itself and all its settings.
//! 
//!
#[doc = include_str!("../README.md")]

pub use self::pedals::pedal_board::PedalBoard;
pub mod dsp;
pub mod utils;
pub mod pedals;
#[macro_use]
extern crate num_derive;

