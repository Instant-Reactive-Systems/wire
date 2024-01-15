//! Common utilities for errors.

use bincode::{Decode, Encode};

use crate::Target;

/// An error directed to a specific target.
///
/// Uses a correlation ID to correlate the error to a request.
#[derive(thiserror::Error, Debug)]
#[error("error '{error}' directed to player '{to:?}'")]
pub struct Error<E> {
	/// The target.
	pub to: Target,
	/// The error.
	pub error: E,
	/// The correlation ID of the request.
	pub corrid: u64,
}

impl<E> bevy_ecs::event::Event for Error<E> where E: bevy_ecs::event::Event {}

impl<E> Error<E>
where
	E: std::error::Error + std::fmt::Debug + Encode + Decode + Clone + PartialEq,
{
	/// Creates a new directed error.
	pub fn new(to: impl Into<Target>, error: impl Into<E>, corrid: u64) -> Self {
		Self {
			to: to.into(),
			error: error.into(),
			corrid,
		}
	}
}

impl<E> Encode for Error<E>
where
	E: Encode,
{
	fn encode<Enc: bincode::enc::Encoder>(&self, encoder: &mut Enc) -> Result<(), bincode::error::EncodeError> {
		self.to.encode(encoder)?;
		self.error.encode(encoder)?;
		self.corrid.encode(encoder)
	}
}

impl<E> Decode for Error<E>
where
	E: Decode,
{
	fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
		Ok(Self {
			to: Target::decode(decoder)?,
			error: E::decode(decoder)?,
			corrid: u64::decode(decoder)?,
		})
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
/// # use bincode::{Encode, Decode};
/// #[derive(thiserror::Error)]
/// pub enum MyError {
/// 	#[error(transparent)]
/// 	Session(#[from] wire::SessionError),
/// 	// ... other variants
/// }
/// ```
#[derive(thiserror::Error, bevy_ecs::prelude::Event, Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionError {
	/// The maximum number of sessions reached.
	#[error("maximum number of sessions reached")]
	MaximumSessionsReached,
	/// The session does not exist.
	#[error("session does not exist")]
	NoSuchSession,
	/// The user is not authenticated.
	#[error("user is not authenticated")]
	Unauthenticated,
}

/// A network error.
///
/// # Usage
/// Use as a transparent variant in custom errors.
///
/// ```
/// # use bincode::{Encode, Decode};
/// #[derive(thiserror::Error)]
/// pub enum MyError {
/// 	#[error(transparent)]
/// 	Network(#[from] wire::NetworkError),
/// 	// ... other variants
/// }
/// ```
#[derive(thiserror::Error, bevy_ecs::prelude::Event, Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkError {
	/// The user has been rate-limited.
	#[error("user has been rate-limited")]
	RateLimited,
	/// Invalid message received.
	#[error("invalid message received")]
	InvalidMessage,
}
