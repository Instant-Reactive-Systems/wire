//! Common utilities for responses.

use crate::Targets;

/// An event that occurred in the system directed towards a particular [`Targets`].
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Res<E> {
	/// The targets that this event is sent to.
	pub targets: Targets,
	/// The event that occurred.
	pub event: TimestampedEvent<E>,
}

impl<E> Res<E> {
	/// Creates a new response.
	pub fn new(targets: impl Into<Targets>, event: impl Into<E>) -> Self {
		Self {
			targets: targets.into(),
			event: TimestampedEvent::new(event),
		}
	}
}

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

/// A timestamped event that occurred in the system directed towards a particular [`Targets`].
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimestampedEvent<E> {
	/// The timestamp of when the event occurred (in ms).
	pub timestamp: i64,
	/// The event that occurred.
	pub event: E,
}

impl<E> TimestampedEvent<E> {
	/// Creates a new event.
	pub fn new(event: impl Into<E>) -> Self {
		Self {
			timestamp: chrono::Utc::now().timestamp_millis(),
			event: event.into(),
		}
	}
}

impl<E> Default for TimestampedEvent<E>
where
	E: Default,
{
	fn default() -> Self {
		Self {
			timestamp: chrono::Utc::now().timestamp_millis(),
			event: Default::default(),
		}
	}
}

impl<E> PartialEq for TimestampedEvent<E>
where
	E: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.event == other.event
	}
}

impl<E> Eq for TimestampedEvent<E> where E: Eq {}

impl<E> std::fmt::Debug for TimestampedEvent<E>
where
	E: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TimestampedEvent")
			.field("timestamp", &self.timestamp)
			.field("event", &self.event)
			.finish()
	}
}

impl<E> Clone for TimestampedEvent<E>
where
	E: Clone,
{
	fn clone(&self) -> Self {
		Self {
			timestamp: self.timestamp.clone(),
			event: self.event.clone(),
		}
	}
}
