use std::hint::black_box;
pub trait Floatting {
    fn float(&self) -> &'static str;
}

pub struct Boat;

impl Floatting for Boat {
    fn float(&self) -> &'static str {
        "Nice boat!"
    }
}

pub struct Duck;

impl Floatting for Duck {
    fn float(&self) -> &'static str {
        "coin coin"
    }
}

pub enum FromEarth {
    Boat(Boat),
    Duck(Duck),
}

impl Floatting for FromEarth {
    fn float(&self) -> &'static str {
        match self {
            FromEarth::Boat(boat) => boat.float(),
            FromEarth::Duck(duck) => duck.float(),
        }
    }
}

#[inline(never)]
pub fn do_static_float<'a, T: Floatting>(floatter: &'a T) -> &'a str {
    floatter.float()
}

#[inline(never)]
pub fn do_dyn_float<'a>(floatter: &'a dyn Floatting) -> &'a str {
    floatter.float()
}

#[inline(never)]
pub fn do_enum_dispatch_float<'a>(floatter: &'a FromEarth) -> &'a str {
    floatter.float()
}

fn main() {
    let boat = Boat;
    let duck = Duck;

    black_box(do_static_float(&boat));
    black_box(do_static_float(&duck));

    black_box(do_dyn_float(&boat));
    black_box(do_dyn_float(&duck));

    black_box(do_enum_dispatch_float(&FromEarth::Boat(boat)));
    black_box(do_enum_dispatch_float(&FromEarth::Duck(duck)));
}
