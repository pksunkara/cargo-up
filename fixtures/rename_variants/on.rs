use upgradee::Enum;

type E = Enum;

fn main() {
    let a = Enum::Orange;
    s(&a);

    let b = E::Orange;
    t(&b);

    if let Enum::Orange = b {}

    let mut e = Enum::Grape(8);

    if let Enum::Grape(g) = e.clone() {}
    if let Enum::Grape(ref mut g) = e {}

    let mut f = Enum::Melon { size: 10 };

    if let Enum::Melon { size } = e.clone() {}
}

fn s(v: &Enum) -> Option<bool> {
    use Enum::*;

    match v {
        Orange => Some(true),
        Grape(_) => Some(false),
        Melon { .. } => Some(false),
        _ => None,
    }
}

fn t(v: &Enum) -> Option<bool> {
    use Enum::*;

    if let Orange = v {
        Some(true)
    } else if let Grape(g) = v {
        Some(false)
    } else if let Melon { size } = v {
        Some(false)
    } else {
        None
    }
}
