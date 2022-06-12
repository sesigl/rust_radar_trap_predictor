# Rust radar trap predictor

Project to learn rust, which gets public available data for radar traps, stores it in some storage to train a prediction model for it. Although rust is not popular for such use-cases, I want to try rust and research if predicting radar traps is possible - for research. Data and outcome is only provided to selected contacts.

## Why Rust

> Rust is a multi-paradigm, general-purpose programming language. It is designed for performance and safety, especially safe concurrency.[12][13] Rust is known for enforcing memory safety — that is, that all references point to valid memory — without requiring the use of a garbage collector or reference counting like other memory-safe languages. Memory safety is enforced by a borrow checker, which tracks object lifetime and variable scope as references are passed throughout the program during compilation. Rust can be used for systems programming with mechanisms for low-level memory management, but also offers high-level features such as functional programming. (Source: https://en.wikipedia.org/wiki/Rust_(programming_language))

This project should help to get experience in Rust:

- how efficient it feels using rust in a container,
- how fast rust is,
- how rust feels to implement use-cases in combination with domain driven design / hexagonal architecture,
- how rust feels for writing tests.

## Why radar traps

I would like to build a model to get evidence if it's possible to predict radar traps. I guess, because there are humans who come up with strategies when and where to send people to measure car speed and violations. If there is an interesting outcome, this can actually be used to setup speed checks with a reasonable variance to make controls more efficient. 

## Legal information 
This project is not about abusing information to avoid radar traps. Everyone can get the data and train a machine learning model.
