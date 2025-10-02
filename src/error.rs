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
	#[error("The maximum number of sessions reached.")]
	MaximumSessionsReached,
	/// The session does not exist.
	#[error("The session does not exist.")]
	NoSuchSession,
	/// The user is not authenticated.
	#[error("The user is not authenticated.")]
	Unauthenticated,
}

#[cfg(feature = "i18n")]
impl i18n::LocalizedDisplay for SessionError {
	fn localize(&self, lang: &i18n::LanguageIdentifier) -> i18n::Message {
		let id = match self {
			Self::MaximumSessionsReached => "session-err-max_reached",
			Self::NoSuchSession => "session-err-no_such_session",
			Self::Unauthenticated => "session-err-unauth",
		};

		crate::i18n::LOCALES.query(lang, &i18n::Query::new(id).with_fallback(true)).unwrap()
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
	#[error("The user has been rate-limited.")]
	RateLimited,
	/// Invalid message received.
	#[error("Invalid message received.")]
	InvalidMessage,
	/// Socket error.
	#[error("Socket error.")]
	SocketError(String),
}

#[cfg(feature = "i18n")]
impl i18n::LocalizedDisplay for NetworkError {
	fn localize(&self, lang: &i18n::LanguageIdentifier) -> i18n::Message {
		use crate::i18n::LOCALES;
		let id = match self {
			Self::RateLimited => "network-err-max_reached",
			Self::InvalidMessage => "network-err-no_such_session",
			Self::SocketError(msg) => return i18n::tr!(lang, "network-err-unauth", "what" = msg),
		};

		crate::i18n::LOCALES.query(lang, &i18n::Query::new(id).with_fallback(true)).unwrap()
	}
}
