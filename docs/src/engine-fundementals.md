# Engine Fundementals

This section of the book will cover ~~some~~ *everything* that is happening
under the hood of the engine. Because this engine is designed to be small, it
shouldn't be too much to get an overview from top to bottom.

There are several different modules that the engine is split into. Each
represents a different core system of what might be needed in a game, and each
tries to encapsulate that functionality as much as possible. This means that if
you are looking for something related to drawing, you should look in the
`graphics` module.

## Interfacing with the engine

To make use of the Micro Game Engine, you will need to implement the `Game`
trait provided by the engine. The reason a trait is used here is that it allows
the engine to expose a very simple API to the user, while the user can then go
on to expand each of these functions however they like. Here is the trait's definition:

```rust
pub trait Game: Sized + 'static {
    const TITLE: &'static str;
    type SaveData: Default;

    fn init(console: &mut Console<Self>) -> Self;

    fn tick(&mut self, dt: f32, console: &mut Console<Self>);

    fn run() { run_with::<Self>() }
}
```

As we can see, there are three functions that need to be implemented.

The first
is `init`, which is called when the game is first started. This is where you
should initialize any data that you need for your game.

The second is `tick`,
which is called every frame. This is where you should update your game state,
and draw to the screen. Often, in other engines, this might be split into
several methods. For example, you might have an `update` method, a `draw`
method, and an `input` method. Micro Game Engine takes a different approach of
just including all this data in one method, and letting the end user decide how
and when to use it.

For the sake of simplicity for demos, some of this book's "case studies" are
implemented by just filling this function with all this separate logic. In your
game, you might want to split this up into several methods, or even several
modules. However, that's an implementation detail that is up to you.

