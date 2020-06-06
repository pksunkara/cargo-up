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

    let struct_member = false;
    let mut e = Struct { struct_member };

    let union_member = false;
    let mut f = Union { union_member };

    let Struct { struct_member } = e.clone();
    let Struct { struct_member: g } = e.clone();
    let Struct {
        ref mut struct_member,
    } = e;

    unsafe {
        let Union { union_member } = f.clone();
        let Union { union_member: h } = f.clone();
        let Union {
            ref mut union_member,
        } = f;
    }
}

fn s(v: &Struct) {
    let v = v.struct_member;
    println!("{}", v);
}
