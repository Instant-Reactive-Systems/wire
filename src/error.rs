//! Common utilities for errors.

use crate::{CorrelationId, Target};

/// An error directed to a specific target.
///
/// Uses a correlation ID to correlate the error to a request.
#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize)]
#[error("error '{error}' directed to player '{to:?}'")]
pub struct Error<E> {
	/// The target.
	pub to: Target,
	/// The error.
	pub error: E,
	/// The correlation ID of the request.
	pub corrid: CorrelationId,
}

impl<E> Error<E> {
	/// Creates a new directed error.
	pub fn new(to: impl Into<Target>, error: impl Into<E>, corrid: CorrelationId) -> Self {
		Self {
			to: to.into(),
			error: error.into(),
			corrid,
		}
	}
}

impl<E> Clone for Error<E>
where
	E: Clone,
{
	fn clone(&self) -> Self {
		Self {
			to: self.to,
			error: self.error.clone(),
			corrid: self.corrid,
		}
	}
}

impl<E> PartialEq for Error<E>
where
	E: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.to == other.to && self.error == other.error && self.corrid == other.corrid
	}
}

impl<E> Eq for Error<E> where E: Eq {}

impl<E> Into<Vec<Error<E>>> for Error<E> {
	fn into(self) -> Vec<Error<E>> {
		vec![self]
	}
}

/// A session error.
///
/// # Usage
/// Use as a transparent variant in custom errors.
///
/// ```
/// #[derive(thiserror::Error, Debug)]
/// pub enum MyError {
/// 	#[error(transparent)]
/// 	Session(#[from] wire::SessionError),
/// 	// ... other variants
/// }
/// ```
#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum SessionError {
	/// The maximum number of sessions reached.
	#[error("wire-err-max_reached")]
	MaximumSessionsReached,
	/// The session does not exist.
	#[error("wire-err-no_such")]
	NoSuchSession,
	/// The user is not authenticated.
	#[error("wire-err-unauth")]
	Unauthenticated,
}

#[cfg(feature = "i18n")]
impl i18n::LocalizedDisplay for SessionError {
	fn localize(&self, lang: &i18n::LanguageIdentifier) -> String {
		use fluent_templates::Loader;
		match self {
			Self::MaximumSessionsReached => crate::i18n::LOCALE.lookup(lang, "wire-err-max_reached"),
			Self::NoSuchSession => crate::i18n::LOCALE.lookup(lang, "wire-err-no_such_session"),
			Self::Unauthenticated => crate::i18n::LOCALE.lookup(lang, "wire-err-unauth"),
		}
	}
}

/// A network error.
///
/// # Usage
/// Use as a transparent variant in custom errors.
///
/// ```
/// #[derive(thiserror::Error, Debug)]
/// pub enum MyError {
/// 	#[error(transparent)]
/// 	Network(#[from] wire::NetworkError),
/// 	// ... other variants
/// }
/// ```
#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum NetworkError {
	/// The user has been rate-limited.
	#[error("wire-err-rate_limited")]
	RateLimited,
	/// Invalid message received.
	#[error("wire-err-invalid_msg")]
	InvalidMessage,
	/// Socket error.
	#[error("wire-err-socket_error")]
	SocketError(String),
}

#[cfg(feature = "i18n")]
impl i18n::LocalizedDisplay for NetworkError {
	fn localize(&self, lang: &i18n::LanguageIdentifier) -> String {
		use fluent_templates::{Loader, fluent_bundle::FluentValue};
		match self {
			Self::RateLimited => crate::i18n::LOCALE.lookup(lang, "wire-err-max_reached"),
			Self::InvalidMessage => crate::i18n::LOCALE.lookup(lang, "wire-err-no_such_session"),
			Self::SocketError(msg) => crate::i18n::LOCALE.lookup_with_args(lang, "wire-err-unauth", &{
				let mut args = std::collections::HashMap::default();
				args.insert("what".to_string(), FluentValue::from(msg));
				args
			}),
		}
	}
}
