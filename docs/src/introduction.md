# Introduction

Welcome to the documentation for the Micro Game Engine! This is a Rust game
engine build to focus on teaching game development in Rust. For this reason,
there are several things that it does a bit differently than other Rust game
engines.

Here are some goals of the engine:

- **Implementing a game without too much complexity**. Many other game engines
  contain a lot of structure that help you with a large project, but can often
  just add cognative overhead to smaller projects.
- **Shipping the engine alongside your game**. If you use this repo as a
  template for your game, then you can (and should!) extend the engine to
  fit your needs. In fact, there are some things that are specifially missing
  from the engine for you to go implement yourself, you can read [this
  chapter](TODO) to find out more about that.
- **Get a prototype running in just a few hours**. This engine is designed to
  make prototypes and toys. Ideally, you should only need to spend a few hours
  to get an idea up and running. After the prototype phase, you should really
  consider moving to a more robust engine. There happen to be a [quite a few
  existing options!](https://arewegameyet.rs/ecosystem/engines/)
- **Some batteries included, you choose the rest**. This engine is designed to
  be a starting point for your game, not a complete solution. It includes
  some basic things like a window, input, and a renderer, but it doesn't enforce
  how you should manage your data. There are [some chapters](TODO) of this book
  that focus on that topic, and you should check them out to learn more about
  the theory.

Here are some non-goals:

- It is not a general purpose game engine. This means that it isn't designed to
  solve every problem that many other game engines do. Some of the things it
  won't include:
    - Networking
    - Physics

