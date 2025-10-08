//! Module containing the target types.

pub use uuid::Uuid;

/// The user ID type.
pub type UserId = Uuid;
/// The session ID type.
pub type SessionId = u32;
/// The bot ID type.
pub type BotId = Uuid;
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
	/// Targets a bot.
	Bot(BotId),
}

impl Target {
	/// Creates a [`Target`] from a user ID and a session ID.
	///
	/// It creates either an anonymous target (if `user_id == ANONYMOUS_USER_ID`) or an authenticated specific target.
	#[deprecated = "Replaced by more clear and intentional methods."]
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

	/// An anonymous or authenticated specific target, depending on the user ID.
	pub fn new_deduced(user_id: UserId, session_id: SessionId) -> Self {
		if user_id == ANON_USER_ID {
			Self::new_anon(session_id)
		} else {
			Self::new_auth_specific(user_id, session_id)
		}
	}

	/// A specific authenticated target.
	pub fn new_auth_specific(user_id: UserId, session_id: SessionId) -> Self {
		Self::Auth(AuthTarget::Specific(user_id, session_id))
	}

	/// An authenticated target.
	pub fn new_auth(user_id: UserId) -> Self {
		Self::Auth(AuthTarget::All(user_id))
	}

	/// An anonymous target.
	pub fn new_anon(session_id: SessionId) -> Self {
		Self::Anon(session_id)
	}

	/// A bot target.
	pub fn new_bot(bot_id: BotId) -> Self {
		Self::Bot(bot_id)
	}

	/// Checks whether the two targets are equal, allowing for same broader authenticated targets to also be equal.
	pub fn weak_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Anon(a), Self::Anon(b)) => a == b,
			(Self::Auth(a), Self::Auth(b)) => a.id() == b.id(),
			(Self::Bot(a), Self::Bot(b)) => a == b,
			_ => false,
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

	/// Checks whether the target is a bot target.
	pub fn is_bot(&self) -> bool {
		match self {
			Self::Bot(_) => true,
			_ => false,
		}
	}

	/// Converts a specific target into a target targetting all user sessions.
	pub fn for_all(&self) -> Target {
		match self {
			Target::Anon(..) => self.clone(),
			Target::Auth(auth_target) => Target::Auth(AuthTarget::All(auth_target.id())),
			Target::Bot(..) => self.clone(),
		}
	}

	/// Returns the target's user ID if it has one.
	pub fn user_id(&self) -> Option<UserId> {
		match self {
			Target::Auth(auth_target) => Some(auth_target.id()),
			_ => None,
		}
	}
}

impl From<(UserId, SessionId)> for Target {
	fn from((user_id, session_id): (UserId, SessionId)) -> Self {
		Self::new_auth_specific(user_id, session_id)
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

impl std::fmt::Display for Target {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Anon(session_id) => write!(f, "anon/{session_id}"),
			Self::Auth(auth_target) => match auth_target {
				AuthTarget::All(user_id) => write!(f, "auth/{user_id}"),
				AuthTarget::Specific(user_id, session_id) => write!(f, "auth/{user_id}/{session_id}"),
			},
			Self::Bot(bot_id) => write!(f, "bot/{bot_id}"),
		}
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

/// An endless (u32-endless) pool of `Target`s.
///
/// Useful in testing.
#[derive(Debug, Clone, Copy)]
pub struct UserPool {
	curr: u32,
}

impl UserPool {
	/// Selects the next unique anon target from the user pool.
	pub fn next_anon(&mut self) -> Target {
		let n = self.curr;
		self.curr += 1;
		Target::new_anon(n)
	}

	/// Selects the next unique bot target from the user pool.
	pub fn next_bot(&mut self) -> Target {
		let n = self.curr;
		self.curr += 1;
		Target::new_bot(Uuid::from_u64_pair(n as u64, n as u64))
	}

	/// Selects the next unique auth target from the user pool.
	pub fn next_auth(&mut self) -> Target {
		let n = self.curr;
		self.curr += 1;
		Target::new_auth_specific(Uuid::from_u64_pair(n as u64, n as u64), 0)
	}
}

impl Default for UserPool {
	fn default() -> Self {
		Self { curr: 1 } // 0 is reserved for anon users
	}
}
