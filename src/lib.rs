//! The goal of this crate is to provide an easy newtype implementation for different types of
//! strongly-typed identifiers.
//!
//! # Motivation
//!
//! Very often, I find myself utilizing many integer-based IDs. In effort to make it strongly
//! typed, one typically uses a "newtype" pattern.
//!
//! ```
//! struct MyId(usize);
//!
//! let id = MyId(1);
//! assert_eq!(id.0, 1);
//! assert_eq!(MyId(id.0 + 10).0, MyId(11).0);
//! ```
//!
//! Notice how you have to access the tuple element with `.0` any time you want to perform any type
//! of operations on the actual integer. One could approach this by implementing `Deref` trait but
//! this is strongly discouraged; see: [Rust Docs](https://doc.rust-lang.org/std/ops/trait.Deref.html),
//! [API Guidelines](https://rust-lang.github.io/api-guidelines/predictability.html#only-smart-pointers-implement-deref-and-derefmut-c-deref).
//!
//! This crate introduces a set of macros implementing certain operations on an ID.
//!
//! # Examples
//!
//! In the simplest case, you only need to a single derive [`Id`](derive.Id.html).
//!
//! ```
//! # use id_derive::Id;
//! #[derive(Id, Debug, PartialEq, Copy, Clone)]
//! struct MyId(usize);
//! // Construct from the inner type.
//! let mut id = MyId::from(1);
//! // Construct inner type from `MyId`.
//! assert_eq!(usize::from(id), 1);
//! // Display.
//! assert_eq!(&id.to_string(), "1");
//! // Add two IDs or inner to ID.
//! assert_eq!(id + MyId(1), MyId(2));
//! assert_eq!(id + 1, MyId(2));
//! id += 1;
//! id += MyId(1);
//! assert_eq!(id, MyId(3));
//! // Subtract
//! assert_eq!(id - MyId(1), MyId(2));
//! assert_eq!(id - 1, MyId(2));
//! id -= 1;
//! id -= MyId(1);
//! assert_eq!(id, MyId(1));
//! // Multiply
//! assert_eq!(id * MyId(2), MyId(2));
//! assert_eq!(id * 2, MyId(2));
//! id *= 2;
//! id *= MyId(2);
//! assert_eq!(id, MyId(4));
//! // Divide
//! assert_eq!(id / MyId(2), MyId(2));
//! assert_eq!(id / 2, MyId(2));
//! id /= 2;
//! id /= MyId(2);
//! assert_eq!(id, MyId(1));
//!
//! ```
//!
//! Alternatively, you may devine only a subset of [available derives](#derives):
//!
//! ```
//! # use id_derive::*;
//! #[derive(Display, FromInner, IntoInner, Add, AddInner)]
//! struct MyId(usize);
//! ```

#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::default_trait_access)]

extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

mod operation;

macro_rules! handle {
    ($s:expr) => {
        proc_macro::TokenStream::from(match $s {
            Ok(tokens) => tokens,
            Err(err) => return ::proc_macro::TokenStream::from(err.to_compile_error()),
        })
    };
    ($($s:expr),*) => {{
        let mut tokens = ::proc_macro2::TokenStream::new();
        $(
            match $s {
                Ok(t) => {
                    tokens.extend(t);
                },
                Err(err) => {
                    return ::proc_macro::TokenStream::from(err.to_compile_error())
                },
            }
        )*
        return proc_macro::TokenStream::from(tokens);
    }};
}

/// Implements `Display`.
#[proc_macro_derive(Display)]
pub fn display(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::display("Display", &input))
}

/// Implements `Add<Self>`.
#[proc_macro_derive(Add)]
pub fn add_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::add_self("Add", &input))
}

/// Implements `Add<T>` where `T` is the type of identifier.
#[proc_macro_derive(AddInner)]
pub fn add_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::add_inner("AddInner", &input))
}

/// Implements `AddAssign<Self>`.
#[proc_macro_derive(AddAssign)]
pub fn add_assign_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::add_assign_self("Add", &input))
}

/// Implements `AddAssign<T>` where `T` is the type of identifier.
#[proc_macro_derive(AddAssignInner)]
pub fn add_assign_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::add_assign_inner("AddInner", &input))
}

/// Implements `Sub<Self>`.
#[proc_macro_derive(Sub)]
pub fn sub_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::sub_self("Sub", &input))
}

/// Implements `Sub<T>` where `T` is the type of identifier.
#[proc_macro_derive(SubInner)]
pub fn sub_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::sub_inner("SubInner", &input))
}

/// Implements `SubAssign<Self>`.
#[proc_macro_derive(SubAssign)]
pub fn sub_assign_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::sub_assign_self("Sub", &input))
}

/// Implements `SubAssign<T>` where `T` is the type of identifier.
#[proc_macro_derive(SubAssignInner)]
pub fn sub_assign_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::sub_assign_inner("SubInner", &input))
}

/// Implements `Mul<Self>`.
#[proc_macro_derive(Mul)]
pub fn mul_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::mul_self("Mul", &input))
}

/// Implements `Mul<T>` where `T` is the type of identifier.
#[proc_macro_derive(MulInner)]
pub fn mul_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::mul_inner("MulInner", &input))
}

/// Implements `MulAssign<Self>`.
#[proc_macro_derive(MulAssign)]
pub fn mul_assign_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::mul_assign_self("MulAssign", &input))
}

/// Implements `MulAssign<T>` where `T` is the type of identifier.
#[proc_macro_derive(MulAssignInner)]
pub fn mul_assign_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::mul_assign_inner("MulAssignInner", &input))
}

/// Implements `Div<Self>`.
#[proc_macro_derive(Div)]
pub fn div_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::div_self("Div", &input))
}

/// Implements `Div<T>` where `T` is the type of identifier.
#[proc_macro_derive(DivInner)]
pub fn div_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::div_inner("DivInner", &input))
}

/// Implements `DivAssign<Self>`.
#[proc_macro_derive(DivAssign)]
pub fn div_assign_self(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::div_assign_self("DivAssign", &input))
}

/// Implements `DivAssign<T>` where `T` is the type of identifier.
#[proc_macro_derive(DivAssignInner)]
pub fn div_assign_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::div_assign_inner("DivAssignInner", &input))
}

/// Implements `From<T>` where `T` is the type of identifier.
#[proc_macro_derive(FromInner)]
pub fn from_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::from_inner("FromInner", &input))
}

/// Implements `From<Self>` for `T` where `T` is the type of identifier.
#[proc_macro_derive(IntoInner)]
pub fn into_inner(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    handle!(operation::into_inner("IntoInner", &input))
}

/// Equivalent to `derive(IntoInner, FromInner)`.
#[proc_macro_derive(Convert)]
pub fn convert(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let from = operation::from_inner("Convert", &input);
    let into = operation::into_inner("Convert", &input);
    handle!(from, into)
}

/// Implement all available traits.
#[proc_macro_derive(Id)]
pub fn id(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let derive_name = "Id";
    handle!(
        operation::from_inner(derive_name, &input),
        operation::into_inner(derive_name, &input),
        operation::add_self(derive_name, &input),
        operation::add_inner(derive_name, &input),
        operation::add_assign_self(derive_name, &input),
        operation::add_assign_inner(derive_name, &input),
        operation::sub_self(derive_name, &input),
        operation::sub_inner(derive_name, &input),
        operation::sub_assign_self(derive_name, &input),
        operation::sub_assign_inner(derive_name, &input),
        operation::mul_self(derive_name, &input),
        operation::mul_inner(derive_name, &input),
        operation::mul_assign_self(derive_name, &input),
        operation::mul_assign_inner(derive_name, &input),
        operation::div_self(derive_name, &input),
        operation::div_inner(derive_name, &input),
        operation::div_assign_self(derive_name, &input),
        operation::div_assign_inner(derive_name, &input),
        operation::display(derive_name, &input)
    )
}
