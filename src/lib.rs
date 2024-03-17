//! A facade and utility crate for on-the-wire communication.
//!
//! Provides [`Req`], [`Res`] and [`Error`] as a generic API for req-res communication.
//! Also provides a utility macro for implementing actions and events for use with communication and also [`bevy`].
//!
//! # Example
//! ```
//! #[derive(wire::WireObj)]
//! #[rustfmt::ignore] // rustfmt will mess up the generated code
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum MyEnum {
//! 	Foo { a: u32, b: u32 },
//! 	Bar(u32, u32),
//! }
//! ```
//!
//! [`bevy`]: https://bevyengine.org
//!
//! # Fluent i18n
//! `wire` exports a couple of common error types which use [`project-fluent`] as a
//! way of specifying locales, therefore it exports its translations via [`fluent-templates`].
//!
//! [`project-fluent`]: https://projectfluent.org
//! [`fluent-templates`]: https://github.com/XAMPPRocky/fluent-templates

pub mod error;
pub use error::{Error, NetworkError, SessionError};

pub mod events;
pub use events::{Connected, Disconnected, FirstConnected};

pub mod req;
pub use req::Req;

pub mod res;
pub use res::Res;

pub mod target;
pub use target::{AuthTarget, SessionId, Target, Targets, UserId, ANON_USER_ID, SYSTEM_USER_ID};
pub use wire_macros::WireObj;

#[cfg(feature = "i18n")]
pub mod i18n;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[rustfmt::skip]
	fn test_wire_obj() {
		#[derive(WireObj)]
		#[rustfmt::ignore]
		#[derive(Clone, Debug, PartialEq, Eq)]
		enum Foo {
			A { a: i32, b: String },
			B { horse: usize },
		}

		let a = A { a: 0xa, b: "b".to_string() };
		let b = B { horse: 42 };
		let foo_a = Foo::A { a: 0xa, b: "b".to_string() };
		let foo_b = Foo::B { horse: 42 };
		println!("{:?}", a.clone());
		println!("{:?}", b.clone());
		println!("{:?}", foo_a.clone());
		println!("{:?}", foo_b.clone());
	}
}
