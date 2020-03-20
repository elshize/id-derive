use id_derive::Id;

#[derive(Id)]
union Test {
    f1: u32,
    f2: u32,
}

fn main() {}
