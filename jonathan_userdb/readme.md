The fundamental issue is how to deal with the fact that this all needs to be very multithreaded.

Possible options:
- use database as lazy_static
- rocket::State https://api.rocket.rs/v0.4/rocket/struct.State.html
- send messages with channel

This example uses Rocket https://rocket.rs/ and it's way to use a `State`. The only "fancy" thing it that the HashMap used for message storage is wrapped in a Mutex (Mutually exclusive) - https://doc.rust-lang.org/book/ch16-03-shared-state.html