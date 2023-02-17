use domains::models::Cat;

fn main() {
    let cat = Cat {
        name: "kiwi".into(),
    };
    println!("{:?}", cat);
}
