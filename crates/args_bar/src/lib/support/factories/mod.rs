use factori::{create, factori};

/// # Factory Prelude
///
/// Contains all of the public factory contents
/// under the `crate::support::factories` module.
///
pub mod prelude {
    pub use super::*;
    pub use factori::create;
}

use crate::models::*;

factori!(InputParameter, {
    default {
        name = "test".into(),
        desc = "test desc".into(),
        default = "".into(),
        required = true,
        encoding = Default::default(),
    }

    mixin string {
        encoding = Encoding::String,
    }

    mixin numeric {
        encoding = Encoding::Numeric,
    }

    mixin boolean {
        encoding = Encoding::Boolean,
    }

    mixin integer {
        encoding = Encoding::Integer,
    }
});

factori!(InputValue, {
    default {
        param = default_input_param(),
        buffer = String::new(),
        touched = false,
    }
});

fn default_input_param() -> InputParameter {
    create!(InputParameter)
}
