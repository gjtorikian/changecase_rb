use changecase::Case;
use changecase::ChangeCase;

use magnus::{class, define_class, function, method, prelude::*, Error};

#[magnus::wrap(class = "ChangecaseRb")]
struct Changecase {
    str: String,
}

impl Changecase {
    fn new(str: String) -> Self {
        Self { str }
    }

    fn to_alt_case(&self) -> String {
        self.str.to_altcase(Case::Upper)
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let class = define_class("ChangecaseRb", class::object())?;
    class.define_singleton_method("new", function!(Changecase::new, 1))?;
    class.define_method("to_alt_case", method!(Changecase::to_alt_case, 0))?;
    Ok(())
}
