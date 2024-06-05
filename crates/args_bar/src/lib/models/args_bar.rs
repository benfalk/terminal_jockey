use super::{InputParameter, InputValue};

// ANCHOR: model
/// Represents the current state of input
#[derive(Debug)]
pub struct ArgsBar {
    /// All of the inputs to track
    inputs: Vec<InputValue>,

    /// input actively receiving data
    current: Option<usize>,
}
// ANCHOR_END: model

impl ArgsBar {
    pub fn new(params: &[InputParameter]) -> Self {
        let inputs: Vec<_> = params.iter().map(InputValue::new).collect();
        let current = None;
        Self { inputs, current }
    }

    /// Cyles forward to the next available input parameter.
    ///
    /// If currently no input is selected and one is available
    /// the first input parameter is selected.  Also, if you
    /// try to toggle past the end of the list it wraps back
    /// around to selecting `None`.
    ///
    pub fn toggle_next(&mut self) {
        if self.inputs.is_empty() {
            return;
        }
        let last_index = self.inputs.len() - 1;
        match self.current {
            Some(idx) if idx == last_index => self.current = None,
            Some(idx) if idx < last_index => self.current = Some(idx + 1),
            None => self.current = Some(0),
            _ => self.current = None,
        }
    }

    /// Cyles backward to the prior available input parameter
    ///
    /// If currently no input is selected it wraps back to the
    /// end of the avialable inputs for selection.  Once at the
    /// first selection if you cycle backwards it selects `None`.
    ///
    pub fn toggle_prev(&mut self) {
        if self.inputs.is_empty() {
            return;
        }
        match self.current {
            Some(0) => self.current = None,
            Some(idx) => self.current = Some(idx - 1),
            None => self.current = Some(self.inputs.len() - 1),
        }
    }

    /// Active Input (Mutable)
    ///
    /// Returns a mutable reference to the input which is
    /// actively selected.
    ///
    pub fn active_input_mut(&mut self) -> Option<&mut InputValue> {
        self.current.and_then(|idx| self.inputs.get_mut(idx))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::support::factories::prelude::*;

    #[test]
    fn working_with_args_bar() {
        let params = [
            create!(InputParameter, name: "foo".into()),
            create!(InputParameter, name: "bar".into()),
        ];
        let mut args_bar = ArgsBar::new(&params);

        assert_eq!(args_bar.inputs[0].param.name.as_ref(), "foo");
        assert_eq!(args_bar.inputs[0].buffer, "");

        assert_eq!(args_bar.inputs[1].param.name.as_ref(), "bar");
        assert_eq!(args_bar.inputs[1].buffer, "");

        assert_eq!(args_bar.current, None);

        args_bar.toggle_next();
        assert_eq!(args_bar.current, Some(0));
        if let Some(input) = args_bar.active_input_mut() {
            input.push_char('B');
        }
        assert_eq!(args_bar.inputs[0].buffer, "B");

        args_bar.toggle_next();
        assert_eq!(args_bar.current, Some(1));

        args_bar.toggle_next();
        assert_eq!(args_bar.current, None);

        args_bar.toggle_next();
        assert_eq!(args_bar.current, Some(0));

        args_bar.toggle_prev();
        assert_eq!(args_bar.current, None);

        args_bar.toggle_prev();
        assert_eq!(args_bar.current, Some(1));

        args_bar.toggle_prev();
        assert_eq!(args_bar.current, Some(0));

        args_bar.toggle_prev();
        assert_eq!(args_bar.current, None);
    }
}
