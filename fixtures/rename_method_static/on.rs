use upgradee::{Enum, Struct, Union};

type S = Struct;
type E = Enum;
type U = Union;

fn main() {
    let a = Struct::print();
    let b = S::print();

    let c = Enum::talk();
    let d = E::talk();

    let e = Union::eat();
    let f = U::eat();
}
