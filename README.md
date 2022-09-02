# Rust Ray Tracing In One Weekend

![A screenshot of the running application showing spheres composed of various materials](doc/screenshot-chapter-12.png)

An attempt to translate Peter Shirley's [_Ray Tracing In One
Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) into Rust.

This is intended to scratch a couple of itches. I wanted a project which:

- let me learn some Rust.
- could produce something visual.

## Development

### Dependencies

#### Rust

Last successfully compiled and run using rust toolchain 1.63.0.

#### SDL2

Note that this uses [SDL2](https://github.com/Rust-SDL2/rust-sdl2) to display a window, draw pixels and display the
result, rather than output the image data to stdout as described in the book. SDL2 requires some set up in your local
environment before you can run this code. See the README of the linked SDL2 repository for installation instructions for
various platforms.

### Running

`cargo run` will compile and run the code, but the rendering will be very slow.

`cargo run --release` will render faster at the cost of a slower compilation time. I have found this almost always worth
doing.

## Progress

So far I have covered chapters 1-12 of the book. This means I can draw an image containing diffuse, metallic and
dielectric spheres, and control the position and orientation of the camera used to render the scene. The results can be
seen above. I can also now simulate a lens, giving depth of field or "defocus blur" as the book calls it.

There are three books in [this series](https://raytracing.github.io/). I plan to finish at least the first book in the
series, admittedly not in just one weekend.

## Notes on Rust

I have deviated from the exact approach shown in the book to try to use more idiomatic Rust.The main example is the use
of Rust enums. The hit method in the book returned a boolean to indicate whether a hit occurred, and took in a pointer
to a hit record to provide the information about the hit if there was one. In Rust, we can instead make use of an enum.
We can return either a Hit or a Miss, and include the information about the hit in the Hit enum value. This ends up
making the code for handling the result of this method quite nice.

I have implemented various traits for a Vec3 struct to reflect the Vec3 class defined in the book. There are almost
certainly more complete linear algebra crates out there, but this was a fun exercise to play with traits. I also plan to
use this module to learn how to write unit tests in Rust, since testing most of the vector operations should be
straightforward.

## Performance

I am not sure what performance I should expect from this code. I don't even know how long it takes for the C++ code in
the book to produce an image. Performance improves considerably when compiled with cargo's `--release` option (as
promised by the Rust docs). I am far from a Rust expert, and might have made some egregious performance fumbles in here
somewhere.

I present the canvas regularly during rendering in lieu of a progress bar. This might be bad for performance, but I
enjoy the effect.

## Correctness

I am unsure about the correctness of my dielectric material. My refractive index appears to be off from the book by a
factor of 2, and I am struggling to reproduce the "hollow glass sphere" from chapter 10.5.