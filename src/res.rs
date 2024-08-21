//! Common utilities for responses.

use crate::Targets;

/// An event that occurred in the system directed towards a particular [`Targets`].
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Res<E> {
	/// The targets that this event is sent to.
	pub targets: Targets,
	/// The event that occurred.
	pub event: E,
}

impl<E> Res<E> {
	/// Creates a new [`Res`] event.
	pub fn new(targets: impl Into<Targets>, event: impl Into<E>) -> Self {
		Self {
			targets: targets.into(),
			event: event.into(),
		}
	}
}

impl<E> bevy_ecs::event::Event for Res<E> where E: bevy_ecs::event::Event {}

impl<E> PartialEq for Res<E>
where
	E: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.targets == other.targets && self.event == other.event
	}
}

impl<E> Eq for Res<E> where E: Eq {}

impl<E> std::fmt::Debug for Res<E>
where
	E: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Res").field("targets", &self.targets).field("event", &self.event).finish()
	}
}

impl<E> Clone for Res<E>
where
	E: Clone,
{
	fn clone(&self) -> Self {
		Self {
			targets: self.targets.clone(),
			event: self.event.clone(),
		}
	}
}

impl<E> Into<Vec<Res<E>>> for Res<E> {
	fn into(self) -> Vec<Res<E>> {
		vec![self]
	}
}
