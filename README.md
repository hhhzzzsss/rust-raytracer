# Rust Raytracer
A raytracer made in Rust

## Group Info
* Group name: Raytracers
* Members:
  * harryyz2
  * cheny8
  * athisht2
  * jizheng4

## Project Introduction
Our group has decided to work on a raytracer implementation in Rust. Raytracing is a powerful but computationally heavy rendering method. Nonetheless, raytracing is very easy to parallelize, so we will hopefully be able to make use of Rust's parallel programming features.

Our goals currently include
* Make a working raytracer that can output rendered images of 3D scenes that we create
* Implement various types of objects that we can add to our 3D scenes, such as spheres, cubes, triangles, (and even full meshes, if we have time).

## System Overview
* Implement a ray struct that holds various relevant information about a ray
* Implement an "intersectable" trait for objects that can be intersected by rays
* Implement a method that traces a single ray, including calculating the correct intersection as well as subsequent ray bounces, and then returning a color or value that can be used to determine color
* Implement a camera object that can do stochastic sampling of rays

## Possible Challenges
* Rust functions quite differently from other languages, and since we're used to object-oriented methods, it might be a bit difficult to get used to the way we design our structs in Rust.
* Debugging the little errors can get quite painful when your code grows in complexity
