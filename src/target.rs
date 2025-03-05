//! Module containing the target types.

pub use uuid::Uuid;

/// The user ID type.
pub type UserId = Uuid;
/// The session ID type.
pub type SessionId = u32;
/// The correlation ID type.
pub type CorrelationId = Uuid;
/// The user ID of an anonymous target.
pub const ANON_USER_ID: UserId = UserId::nil();

/// An enum representing an authenticated target.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
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
		let user_id = UserId::new_v4();
		Self::Auth(AuthTarget::Specific(user_id, 0))
	}

	/// Creates a [`Target`] from an automatically generated user ID and a session ID.
	///
	/// Useful in cases of testing.
	pub fn new_random_with_session(session_id: SessionId) -> Self {
		let user_id = UserId::new_v4();
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
			Target::Anon(_) => self.clone(),
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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum Targets {
	/// Targets all sessions.
	All,
	/// Targets only a set of specific targets.
	Few(Vec<Target>),
}

/// An endless (u64-endless) pool of `Target`s.
///
/// Useful in testing.
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
