use upgradee::{Struct, Union};

type S = Struct;
type U = Union;

fn main() {
    let mut a = Struct {
        struct_member: true,
    };

    a.struct_member = false;
    s(&a);

    let b = S {
        struct_member: false,
    };
    s(&b);

    let mut c = Union { union_member: true };

    c.union_member = true;

    let z = |v: &Union| {
        let v = unsafe { v.union_member };
        println!("{}", v);
    };

    z(&c);

    let d = U {
        union_member: false,
    };
    z(&d);
}

fn s(v: &Struct) {
    let v = v.struct_member;
    println!("{}", v);
}
