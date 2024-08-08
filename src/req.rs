//! Common utilities for requests.

use crate::*;

/// A request by a target (anonymous or authenticated) to perform an action.
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Req<A> {
	/// The target that sent the request.
	pub from: Target,
	/// The action that the target wants to perform.
	pub action: A,
	/// The correlation ID of the request.
	pub corrid: CorrelationId,
}

impl<A> Req<A> {
	/// Creates a new request.
	pub fn new(from: impl Into<Target>, action: impl Into<A>, corrid: CorrelationId) -> Self {
		Self {
			from: from.into(),
			action: action.into(),
			corrid,
		}
	}
}

impl<A> bevy_ecs::event::Event for Req<A> where A: bevy_ecs::event::Event {}

impl<A> PartialEq for Req<A>
where
	A: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.from == other.from && self.action == other.action && self.corrid == other.corrid
	}
}

impl<A> Eq for Req<A> where A: Eq {}

impl<A> std::hash::Hash for Req<A>
where
	A: std::hash::Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.from.hash(state);
		self.action.hash(state);
		self.corrid.hash(state);
	}
}

impl<A> std::fmt::Debug for Req<A>
where
	A: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Req")
			.field("from", &self.from)
			.field("action", &self.action)
			.field("corrid", &self.corrid)
			.finish()
	}
}

impl<A> Clone for Req<A>
where
	A: Clone,
{
	fn clone(&self) -> Self {
		Self {
			from: self.from,
			action: self.action.clone(),
			corrid: self.corrid.clone(),
		}
	}
}

impl<E> Into<Vec<Req<E>>> for Req<E> {
	fn into(self) -> Vec<Req<E>> {
		vec![self]
	}
}
