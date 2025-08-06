# wire
A facade crate for a unified way of declaring an action-event-error protocol with internal request correlation tracking. Used for real-time communication via sockets.

Provides `Req`, `Res` and `Error` as a generic API for req-res communication.
Also provides a utility macro for implementing actions and events for use with communication.

# Adding as a dependency

```toml
[dependencies]
wire = { git = "https://github.com/Instant-Reactive-Systems/wire.git" }
```
