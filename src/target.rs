//! Module containing the target types.

/// A wrapper around a Uuid providing bincode support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Uuid(uuid::Uuid);

impl Uuid {
	/// A nil uuid.
	pub const fn nil() -> Self {
		Self(uuid::Uuid::nil())
	}

	/// Generates a new random uuidv4.
	pub fn new_random() -> Self {
		Self(uuid::Uuid::new_v4())
	}

	/// Creates a new specific uuidv4 from a u64 pair.
	pub const fn from_u64_pair(high: u64, low: u64) -> Self {
		Self(uuid::Uuid::from_u64_pair(high, low))
	}
}

impl std::fmt::Display for UserId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.as_hyphenated().fmt(f)
	}
}

#[cfg(feature = "bincode")]
impl bincode::Encode for UserId {
	fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
		self.0.as_bytes().encode(encoder)
	}
}

#[cfg(feature = "bincode")]
impl bincode::Decode for UserId {
	fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
		let user_id = <[u8; 16]>::decode(decoder)?;
		let user_id = uuid::Uuid::from_slice(&user_id).map_err(|err| bincode::error::DecodeError::OtherString(format!("{:?}", err)))?;
		Ok(Self(user_id))
	}
}

#[cfg(feature = "bincode")]
impl<'de> bincode::BorrowDecode<'de> for UserId {
	fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
		let user_id = <[u8; 16]>::borrow_decode(decoder)?;
		let user_id = uuid::Uuid::from_slice(&user_id).map_err(|err| bincode::error::DecodeError::OtherString(format!("{:?}", err)))?;
		Ok(Self(user_id))
	}
}

/// The user ID type.
pub type UserId = Uuid;
/// The session ID type.
pub type SessionId = u32;
/// The correlation ID type.
pub type CorrelationId = Uuid;
/// The user ID of an anonymous target.
pub const ANON_USER_ID: UserId = UserId::nil();

/// An enum representing an authenticated target.
#[derive(bevy_ecs::prelude::Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuthTarget {
	/// Targets all sessions of a user.
	All(UserId),
	/// Targets only a specific session of a user.
	Specific(UserId, SessionId),
}

impl AuthTarget {
	/// Returns the user ID of the target.
	pub fn id(&self) -> UserId {
		match self {
			Self::Specific(user_id, _) => *user_id,
			Self::All(user_id) => *user_id,
		}
	}
}

impl Into<Target> for AuthTarget {
	fn into(self) -> Target {
		Target::Auth(self)
	}
}

/// An enum representing a target.
///
/// A target can be either a source or a destination of a particular message.
#[derive(bevy_ecs::prelude::Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Target {
	/// Targets an anonymous session.
	Anon(SessionId),
	/// Targets an authenticated session.
	Auth(AuthTarget),
}

impl Target {
	/// Creates a [`Target`] from a user ID and a session ID.
	///
	/// It creates either an anonymous target (if `user_id == ANONYMOUS_USER_ID`) or an authenticated specific target.
	pub fn new(user_id: UserId, session_id: SessionId) -> Self {
		if user_id == ANON_USER_ID {
			Self::Anon(session_id)
		} else {
			Self::Auth(AuthTarget::Specific(user_id, session_id))
		}
	}

	/// Creates a [`Target`] from an automatically generated user ID and defaulted session ID (0).
	///
	/// Useful in cases of testing.
	pub fn new_random() -> Self {
		let user_id = UserId::new_random();
		Self::Auth(AuthTarget::Specific(user_id, 0))
	}

	/// Creates a [`Target`] from an automatically generated user ID and a session ID.
	///
	/// Useful in cases of testing.
	pub fn new_random_with_session(session_id: SessionId) -> Self {
		let user_id = UserId::new_random();
		Self::Auth(AuthTarget::Specific(user_id, session_id))
	}

	/// Checks whether the two targets are equal, allowing for same broader authenticated targets to also be equal.
	pub fn weak_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Anon(session_id), Self::Anon(other_session_id)) => session_id == other_session_id,
			(Self::Auth(a), Self::Auth(b)) => a.id() == b.id(),
			_ => false,
		}
	}

	/// Returns the user ID of the target.
	pub fn id(&self) -> UserId {
		match self {
			Self::Anon(_) => ANON_USER_ID,
			Self::Auth(auth) => auth.id(),
		}
	}

	/// Checks whether the target is an anonymous target.
	pub fn is_anon(&self) -> bool {
		match self {
			Self::Anon(_) => true,
			_ => false,
		}
	}

	/// Checks whether the target is an authenticated target.
	pub fn is_auth(&self) -> bool {
		match self {
			Self::Auth(_) => true,
			_ => false,
		}
	}

	/// Converts a specific target into a target targetting all user sessions.
	pub fn for_all(&self) -> Target {
		match self {
			Target::Anon(_) => panic!("Target::for_all() called on Anon target"),
			Target::Auth(auth_target) => Target::Auth(AuthTarget::All(auth_target.id())),
		}
	}
}

impl From<(UserId, SessionId)> for Target {
	fn from((user_id, session_id): (UserId, SessionId)) -> Self {
		Self::new(user_id, session_id)
	}
}

impl Into<Targets> for Target {
	fn into(self) -> Targets {
		Targets::Few(vec![self])
	}
}

impl Into<Targets> for Vec<Target> {
	fn into(self) -> Targets {
		Targets::Few(self)
	}
}

impl Into<Targets> for AuthTarget {
	fn into(self) -> Targets {
		Targets::Few(vec![self.into()])
	}
}

impl Into<Targets> for Vec<AuthTarget> {
	fn into(self) -> Targets {
		Targets::Few(self.into_iter().map(Into::into).collect())
	}
}

/// The targets that a message can be sent to.
#[derive(bevy_ecs::prelude::Event, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Targets {
	/// Targets all sessions.
	All,
	/// Targets only a set of specific targets.
	Few(Vec<Target>),
}

/// An endless (u64-endless) pool of `Target`s.
#[derive(Debug, Clone, Copy)]
pub struct UserPool {
	curr: u64,
}

impl UserPool {
	/// Selects the next unique target from the user pool.
	pub fn next(&mut self) -> Target {
		let n = self.curr;
		self.curr += 1;
		Target::new(Uuid::from_u64_pair(n, n), 0)
	}
}

impl Default for UserPool {
	fn default() -> Self {
		Self { curr: 1 } // 0 is reserved for anon users
	}
}
