use upgradee::Struct;

fn main() {
    let a = Struct {};

    a.print();

    app(&a);
}

fn app(v: &Struct) {
    v.print();
}
