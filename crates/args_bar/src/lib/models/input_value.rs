use super::InputParameter;
use std::borrow::BorrowMut;

// ANCHOR: model
/// Keeps track of a provided value against a
/// target `InputParameter`
#[derive(Debug)]
pub struct InputValue {
    /// information on data being collected
    pub(crate) param: InputParameter,

    /// the current buffered value of input
    pub(crate) buffer: String,

    /// true if the input has been interacted with
    pub(crate) touched: bool,
}
// ANCHOR_END: model

impl InputValue {
    /// Creates a new input value to work with from a provided
    /// [`InputParameter`].  The input starts off as un-touched.
    ///
    pub fn new(param: &InputParameter) -> Self {
        Self {
            param: param.clone(),
            buffer: String::new(),
            touched: false,
        }
    }

    /// Pushes a character to the end of the input buffer.
    /// The encoding in the end determines what input is
    /// permitted and how to interpret it.
    ///
    /// [`InputValue::has_been_touched`] will report `true`
    ///
    pub fn push_char(&mut self, character: char) {
        self.touch();

        self.param
            .encoding
            .push_char(&mut self.buffer, character)
            .ok();
    }

    /// Pops a character off the input buffer
    ///
    /// Nothing by default will ever prevent a character
    /// from being dropped; however, this can change a
    /// formerly valid input to become invalid.
    ///
    /// Returns the char that was removed from the end
    /// in case you want to store a history of erased
    /// input.
    ///
    /// [`InputValue::has_been_touched`] will report `true`
    ///
    pub fn pop_char(&mut self) -> Option<char> {
        self.touch();
        self.buffer.pop()
    }

    /// Mark input as touched
    ///
    /// In some cases you may want to mark an input as
    /// interacted with even if no input was added.  This
    /// can be used for triggering validation that is
    /// skipped over or identifying which inputs have
    /// been highlighted.
    ///
    /// [`InputValue::has_been_touched`] will report `true`
    ///
    pub fn touch(&mut self) {
        self.touched = true;
    }

    /// Reset [`InputValue`] to it's initial state
    ///
    /// Sets the state back to when it was originally
    /// created.  Good for implementing a quick "delete"
    /// of an input as [`InputValue::has_been_touched`]
    /// is reset as well.
    ///
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.touched = false;
    }

    /// Determines if this input has been in contact
    /// with some form of input.  When an input is
    /// orginally created or [`reset`] this will
    /// report `false`.  These following methods however
    /// will cause this to report `true`:
    ///
    /// - [`InputValue::touch`]
    /// - [`InputValue::push_char`]
    /// - [`InputValue::pop_char`]
    ///
    /// [`reset`]: #method.reset
    ///
    pub fn has_been_touched(&self) -> bool {
        self.touched
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::factories::prelude::*;

    #[test]
    fn building_an_input_value() {
        let param = create!(InputParameter);
        let input = InputValue::new(&param);

        assert!(input.buffer.is_empty());
        assert!(!input.touched);
        assert_eq!(input.param.name, param.name);
    }
}
