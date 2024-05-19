enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // `use`することで絶対名でなくとも使用可能になる。
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // `Work`の中の名前をすべて`use`する
    use crate::Work::*;

    // `use`しているため、`Status::Poor`と書いていることに等しい
    let status = Poor;
    // `Work::Civilian`に等しい
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        // `use`しているのでスコープを明示していない
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        // こちらも同じ
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}