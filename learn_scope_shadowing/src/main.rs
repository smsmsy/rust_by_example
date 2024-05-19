fn main() {
    // {} に囲まれた範囲でスコープが定まる
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    println!("outer long: {}", long_lived_binding);

    
    let shadowed_binding = 1;
    {
        // この時点では 1 が出力される
        println!("before being shadowed: {}", shadowed_binding);

        // 同名の変数を宣言すると上書きされる
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    // ブロックを抜けると変数の上書きがなかったことになる
    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
