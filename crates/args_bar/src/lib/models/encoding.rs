// ANCHOR: model
/// Describes the type of value for an `Argument`
#[derive(Debug, Default, Copy, Clone)]
pub enum Encoding {
    #[default]
    String,
    Integer,
    Numeric,
    Boolean,
}
// ANCHOR_END: model

impl Encoding {
    pub fn push_char(&self, buffer: &mut String, ch: char) -> Result<(), String> {
        push_guard(self, buffer, ch)?;
        buffer.push(ch);
        Ok(())
    }
}

fn push_guard(encoding: &Encoding, buffer: &str, ch: char) -> Result<(), String> {
    match encoding {
        Encoding::Integer if !ch.is_ascii_digit() => Err(format!("`{}` is not a digit", ch)),
        Encoding::Boolean => match (buffer, ch) {
            ("", val) if val != 'f' && val != 't' => {
                Err(format!("Expecting `f` or `t`, not `{}`", val))
            }

            // Spelling out `false`
            ("f", val) if val != 'a' => Err(format!("Expecting `a`, not `{}`", val)),
            ("fa", val) if val != 'l' => Err(format!("Expecting `l`, not `{}`", val)),
            ("fal", val) if val != 's' => Err(format!("Expecting `s`, not `{}`", val)),
            ("fals", val) if val != 'e' => Err(format!("Expecting `e`, not `{}`", val)),

            // Spelling out `true`
            ("t", val) if val != 'r' => Err(format!("Expecting `r`, not `{}`", val)),
            ("tr", val) if val != 'u' => Err(format!("Expecting `u`, not `{}`", val)),
            ("tru", val) if val != 'e' => Err(format!("Expecting `e`, not `{}`", val)),

            // `false` already spelled out
            ("false", val) => Err(format!("`false` set, not expecting {}", val)),

            // `true` already spelled out
            ("true", val) => Err(format!("`true` set, not expecting {}", val)),

            // something is really messed up
            (state, val) => Err(format!("Bad State: {}, char: {}", state, val)),
        },
        Encoding::Numeric => match ch {
            val if val.is_ascii_digit() => Ok(()),
            '.' if !buffer.contains('.') => Ok(()),
            val => Err(format!("Non-numeric not permitted: {}", val)),
        },
        _ => Ok(()),
    }
}
