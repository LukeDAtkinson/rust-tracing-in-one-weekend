# Rust Ray Tracing In One Weekend

An attempt to translate [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## Progress

So far I have covered chapters 1-8 of the book. This means I can draw an image containing two diffuse spheres.

There are three books in [this series](https://raytracing.github.io/). I plan to finish at least the first book in the
series, admittedly not in just one weekend.

## SDL2

Note that I am using [SDL2](https://github.com/Rust-SDL2/rust-sdl2) to display a window, draw pixels and display the
result, rather than output the image data to stdout as described in the book. This was mostly just because I wanted to
try out creating and drawing to a window in Rust. SDL2 requires some set up in your local environment before you can run
this code. See the README of the linked SDL2 repository for installation instructions for various platforms.

I present the canvas regularly during rendering in lieu of a progress bar. This might be bad for performance, but I
enjoy the effect.

## Notes on Rust

I have deviated from the exact approach shown in the book to use more idiomatic Rust.The main example is the use of Rust
enums. The hit method in the book returned a boolean to indicate whether a hit occurred, and took in a pointer to a hit
record to provide the information about the hit if there was one. In Rust, we can instead make use of an enum. We can
return either a Hit or a Miss, and include the information about the hit in the Hit enum value. This ends up making the
code for handling the result of this method quite nice.

I have implemented various traits for a Vec3 struct to reflect the Vec3 class defined in the book. There are almost
certainly more complete linear algebra crates out there, but this was a fun exercise to play with traits.

## Performance

I am not sure what performance I should expect from this code. I don't even know how long it takes for the C++ code in
the book to produce an image. However, it certainly feels very slow. I am far from a Rust expert, and might have made
some egregious performance fumbles in here somewhere.