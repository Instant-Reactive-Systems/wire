//! Common events used by the server and/or client.

use crate::*;

/// Event indicating that a user was authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Authenticated;

/// Event indicating that a user was unauthenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Unauthenticated;

/// Marker type of a connection event that hasn't been mapped into any category yet.
#[derive(Debug, Clone, Copy)]
pub struct Undetermined;

/// Event indicating a user connected to the server.
///
/// It is generic to allow for muxing into different handlers.
pub struct Connected<M> {
	/// The user id of the user.
	pub user_id: UserId,
	/// The session id of the user.
	pub session_id: SessionId,
	_phantom: std::marker::PhantomData<M>,
}

impl<M> Connected<M> {
	/// Creates a new [`Connected`] event.
	pub fn new(user_id: UserId, session_id: SessionId) -> Self {
		Self {
			user_id,
			session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> PartialEq for Connected<M> {
	fn eq(&self, other: &Self) -> bool {
		self.user_id == other.user_id && self.session_id == other.session_id
	}
}

impl<M> Eq for Connected<M> {}

impl<M> std::hash::Hash for Connected<M> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.user_id.hash(state);
		self.session_id.hash(state);
	}
}

impl<M> std::fmt::Debug for Connected<M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Connected")
			.field("user_id", &self.user_id)
			.field("session_id", &self.session_id)
			.finish()
	}
}

impl<M> Clone for Connected<M> {
	fn clone(&self) -> Self {
		Self {
			user_id: self.user_id,
			session_id: self.session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> Copy for Connected<M> {}

/// Event indicating a user disconnected from the server.
///
/// It is generic to allow for muxing into different handlers.
pub struct Disconnected<M> {
	/// The user id of the user.
	pub user_id: UserId,
	/// The session id of the user.
	pub session_id: SessionId,
	_phantom: std::marker::PhantomData<M>,
}

impl<M> Disconnected<M> {
	/// Creates a new [`Disconnected`] event.
	pub fn new(user_id: UserId, session_id: SessionId) -> Self {
		Self {
			user_id,
			session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> PartialEq for Disconnected<M> {
	fn eq(&self, other: &Self) -> bool {
		self.user_id == other.user_id && self.session_id == other.session_id
	}
}

impl<M> Eq for Disconnected<M> {}

impl<M> std::hash::Hash for Disconnected<M> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.user_id.hash(state);
		self.session_id.hash(state);
	}
}

impl<M> std::fmt::Debug for Disconnected<M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Disconnected")
			.field("user_id", &self.user_id)
			.field("session_id", &self.session_id)
			.finish()
	}
}

impl<M> Clone for Disconnected<M> {
	fn clone(&self) -> Self {
		Self {
			user_id: self.user_id,
			session_id: self.session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> Copy for Disconnected<M> {}

/// Event indicating a user connected to the server without having a previous session active.
///
/// It is generic to allow for muxing into different handlers.
pub struct FirstConnected<M> {
	/// The user id of the user.
	pub user_id: UserId,
	/// The session id of the user.
	pub session_id: SessionId,
	_phantom: std::marker::PhantomData<M>,
}

impl<M> FirstConnected<M> {
	/// Creates a new [`Connected`] event.
	pub fn new(user_id: UserId, session_id: SessionId) -> Self {
		Self {
			user_id,
			session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> PartialEq for FirstConnected<M> {
	fn eq(&self, other: &Self) -> bool {
		self.user_id == other.user_id && self.session_id == other.session_id
	}
}

impl<M> Eq for FirstConnected<M> {}

impl<M> std::hash::Hash for FirstConnected<M> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.user_id.hash(state);
		self.session_id.hash(state);
	}
}

impl<M> std::fmt::Debug for FirstConnected<M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("FirstConnected")
			.field("user_id", &self.user_id)
			.field("session_id", &self.session_id)
			.finish()
	}
}

impl<M> Clone for FirstConnected<M> {
	fn clone(&self) -> Self {
		Self {
			user_id: self.user_id,
			session_id: self.session_id,
			_phantom: Default::default(),
		}
	}
}

impl<M> Copy for FirstConnected<M> {}

impl<M> From<Connected<M>> for Target {
	fn from(value: Connected<M>) -> Self {
		if value.user_id == ANON_USER_ID {
			Target::Anon(value.session_id)
		} else {
			Target::Auth(AuthTarget::Specific(value.user_id, value.session_id))
		}
	}
}

impl<M> From<Disconnected<M>> for Target {
	fn from(value: Disconnected<M>) -> Self {
		if value.user_id == ANON_USER_ID {
			Target::Anon(value.session_id)
		} else {
			Target::Auth(AuthTarget::Specific(value.user_id, value.session_id))
		}
	}
}

impl<M> From<FirstConnected<M>> for Target {
	fn from(value: FirstConnected<M>) -> Self {
		if value.user_id == ANON_USER_ID {
			Target::Anon(value.session_id)
		} else {
			Target::Auth(AuthTarget::Specific(value.user_id, value.session_id))
		}
	}
}

impl<M> Into<(UserId, SessionId)> for Connected<M> {
	fn into(self) -> (UserId, SessionId) {
		(self.user_id, self.session_id)
	}
}

impl<M> Into<(UserId, SessionId)> for Disconnected<M> {
	fn into(self) -> (UserId, SessionId) {
		(self.user_id, self.session_id)
	}
}

impl<M> Into<(UserId, SessionId)> for FirstConnected<M> {
	fn into(self) -> (UserId, SessionId) {
		(self.user_id, self.session_id)
	}
}
