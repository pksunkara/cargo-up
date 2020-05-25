use upgradee::Struct;

fn main() {
    let mut a = Struct { member: true };

    a.member = false;

    println!("{}", a.member);
}
