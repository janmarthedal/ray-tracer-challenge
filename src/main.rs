mod approx_eq;
mod canvas;
mod color;
mod tuple;
use tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::new(position, velocity)
}

fn main() {
    // projectile starts one unit above the origin.
    // velocity is normalized to 1 unit/tick.
    let mut p = Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    );
    // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let e = Environment::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.0, 0.0),
    );

    while p.position.y > 0.0 {
        println!("{:?}", p.position);
        p = tick(&e, &p);
    }
}
