use math::big::Int;

fn main() {
    let mut i = Int::default();
    i.set_string("644", 8);

    assert_eq!(i, Int::new(420));
}
