use upgradee::{Enum, Struct, Union};

type S = Struct;
type E = Enum;
type U = Union;

fn main() {
    let a = Struct {};

    a.print();

    let z = |v: Struct| v.print();

    z(a);

    let c = S {};

    c.print();

    let b = Enum::None;

    b.talk();

    let z = |v: Enum| v.talk();

    z(b);

    let d = E::None;

    d.talk();

    let e = Union { y: false };

    e.eat();

    let z = |v: Union| v.eat();

    z(e);

    let f = U { y: false };

    f.eat();
}
