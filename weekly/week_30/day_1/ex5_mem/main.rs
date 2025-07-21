#[repr(C)]
struct MonCapteur {
    id: u16,
    actif: bool,
    valeur: f64,
}

fn main() {
    println!("Align of :{:?}", std::mem::align_of::<MonCapteur>());
    println!("Size of :{:?}", std::mem::size_of::<MonCapteur>());
}
