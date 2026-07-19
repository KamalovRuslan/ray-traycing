# Ray Traycing

A ray tracer implemented in Rust, following ["Ray Tracing in One Weekend"](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

Built as a learning project to explore Rust concepts through graphics programming.

## Output

![render](output.png)

## Features

- Materials: Lambertian (diffuse), Metal (with fuzz), Dielectric (glass)
- Positionable camera with configurable field of view
- Depth of field (aperture simulation)
- Anti-aliasing (multi-sample per pixel)
- Parallel rendering with rayon

## Project Structure

```
src/
  main.rs          — scene generation and parallel render loop
  vector/mod.rs    — Vec3 with arithmetic ops
  ray/mod.rs       — Ray, Sphere, Hit trait, HitList
  camera/mod.rs    — Camera (position, fov, depth of field)
  material/mod.rs  — Material trait, Lambertian, Metal, Dielectric
```
