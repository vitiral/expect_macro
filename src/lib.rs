/* Copyright (c) 2018 Garrett Berg, vitiral@gmail.com
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */
//! **The `expect!` macro**
//!
//! This crate adds the `expect!` macro, which is basically just calling
//! `.expect` on your type but also:
//!
//! - Includes the exact line number of the error
//! - Allows you to specify a custom error message with formatting.
//! - Lazy evaluates error conditions (unlike `result.expect(&format!(...))`)
//!
//! This gives you panic messages like this:
//!
//! ```no_compile
//! thread 'example' panicked at '"expect error"', src/lib.rs:5:5
//! ```
//!
//! As opposed to:
//!
//! ```no_compile
//! thread 'example' panicked at 'called `Result::unwrap()` on an `Err` value: "expect error"', libcore/result.rs:945:5
//! ```
//!
//! # Alternatives
//!
//! If you need to include the `Err` in a custom error message then do this instead:
//!
//! ```rust,should_panic
//! # fn main() {
//! let result = Err("expect error");
//! result.unwrap_or_else(|err| panic!("Got {} but expected 42", err));
//! # }
//! ```

/// Unwrap a result or `panic!` with a message.
///
/// Works with [`Result`] and [`Option`].
///
/// This macro has two forms:
///
/// - `expect!(result)`: calls `panic!("{:#?}", err)` on any unwrapped `Err`/`None`.
/// - `expect!(result, ...)`: calls `panic!(...)` on any unwrapped `Err`/`None`, allowing you to
///   specify your own error formatting. This is recommened when you are using `expect!` with
///   [`Option`]
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
///
/// # Example
///
/// Without format
///
/// ```rust,should_panic
/// #[macro_use] extern crate expect_macro;
/// use expect_macro::*;
///
/// # fn main() {
/// let result = Err("expect error");
/// expect!(result);
/// # }
///
/// // COMPILER OUTPUT:
/// // thread 'example' panicked at '"expect error"', src/lib.rs:5:5
/// ```
///
/// With format
///
/// ```rust,should_panic
/// #[macro_use] extern crate expect_macro;
/// use expect_macro::*;
///
/// # fn main() {
/// let result = Err("expect error");
/// expect!(result, "Some values: {}, {}", 1, 2);
/// # }
///
/// // COMPILER OUTPUT:
/// // thread 'example' panicked at 'Some values: 1, 2', src/lib.rs:5:5
/// ```
#[macro_export]
macro_rules! expect {
    [$result:expr, $($rest:tt)*] => {
        $crate::IntoResult::into_result($result).unwrap_or_else(|_| {
            panic!($($rest)*)
        })
    };
    [$result:expr] => {
        $crate::IntoResult::into_result($result).unwrap_or_else(|e| {
            panic!("{:?}", e)
        })
    };
}

/// Used to ensure either `Option` or `Result` are the `Result` type.
pub trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

impl<T> IntoResult<T, &'static str> for Option<T> {
    fn into_result(self) -> Result<T, &'static str> {
        match self {
            Some(v) => Ok(v),
            None => Err("Got value of None"),
        }
    }
}

#[test]
#[should_panic]
fn expect_panic_bare() {
    expect!(Err("expect error"));
}

#[test]
#[should_panic]
fn regular_panic_bare() {
    let result: Result<(), &str> = Err("expect error");
    result.unwrap();
}

#[test]
#[should_panic]
fn expect_panic_msg() {
    expect!(Err("expect error"), "Some values: {}, {}", 1, 2);
}

#[test]
#[should_panic]
fn sanity_chain() {
    fn foo(a: ()) -> Result<(), &'static str> {
        Ok(a)
    }

    #[allow(unused_variables)]
    fn bar(a: ()) -> Result<(), &'static str> {
        Err("expect error")
    }

    let a = ();
    expect!(bar(expect!(foo(a))));
}

#[test]
#[should_panic]
fn sanity_option_plain() {
    expect!(None);
}

#[test]
#[should_panic]
fn sanity_option_msg() {
    expect!(None, "Got None, expected 42");
}
