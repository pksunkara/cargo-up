use upgradee::OldStruct;

type S = OldStruct;

struct I {
    inner: OldStruct,
}

fn main() {
    let mut a = OldStruct {
        struct_member: true,
    };

    a.struct_member = false;
    s(&a);

    let mut e = OldStruct {
        struct_member: false,
    };

    let OldStruct { struct_member } = e.clone();
}

fn s(v: &OldStruct) {
    match v {
        OldStruct { struct_member } => println!("{}", struct_member),
    };
}
