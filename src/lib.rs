//! _Sometimes you want to throw errors away._
//!
//! This crate does exactly one thing, it adds a new method to the `Result`
//! type that throws away the current error, if there is one, and replaces
//! it with a new value. You can see it in action here:
//!
//! ```
//! use replace_err::ReplaceErr as _;
//!
//! let result = Err(1);
//! let result: Result<(), _> = result.replace_err("hello");
//! assert_eq!(result.unwrap_err(), "hello");
//! ```
//!
//! This is exactly equivalent to calling `Result::map_err` with a closure
//! which ignores the input and returns something else. In fact, that's how
//! `replace_err` is implemented.
//!
//! Most of the time, you _do not_ want to do this. Usually you want to
//! wrap prior errors with new layers to add context, giving you a chain
//! of increasingly-specific and low-level explanations which error
//! reporters can present to the user, or based on which higher-level code
//! can take action.
//!
//! However, there _are_ some cases where you really don't need the
//! underlying error, and `replace_err` provides a convenient way to
//! express that need.

/// Extend `Result` with a `replace_err` method.
pub trait ReplaceErr: private::Sealed {
    /// The type of the `Ok` variant in the `Result`.
    type Ok;

    /// Replace the existing error variant with a new value.
    fn replace_err<F>(self, err: F) -> Result<Self::Ok, F>;
}

impl<T, E> ReplaceErr for Result<T, E> {
    type Ok = T;

    fn replace_err<F>(self, err: F) -> Result<<Self as ReplaceErr>::Ok, F> {
        self.map_err(|_| err)
    }
}

mod private {
	pub trait Sealed {}

	impl<T, E> Sealed for Result<T, E> {}
}

#[cfg(test)]
mod tests {
    use super::ReplaceErr as _;

    #[test]
    fn it_works() {
        // Result<_, i32>
        let result = Err(1);

        // Result<(), &'static str>
        // have to specify the Ok variant here, usually it will be clear from context.
        let result: Result<(), _> = result.replace_err("hello");

        assert_eq!(result.unwrap_err(), "hello");
    }
}
