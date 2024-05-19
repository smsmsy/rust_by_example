// Suppress all warnings from casts which overflow.
// オーバーフローを起こすようなキャストによる警告を無視する。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 何らかの値を符号なしの型（仮にTとする）へキャストすると
    // 値がTに収まるまで、T::MAX + 1 が加算あるいは減算される。

    // 1000 はすでにu16に収まっているため変化しない。
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 水面下では最下位ビットから8bitが使用され、残りの上位ビットが圧縮される形になる。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 正の数では、これは剰余と同じです。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 符号付きの型にキャストする場合、結果は以下の2つを行った場合に等しい
    // 1. 対応する符号なしの型にキャストする。
    // 2. 2の補数(two's complement)をとる

    // すでに収まっている場合はそのままです。
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128をu8にキャストすると128となる。128の8ビットにおける補数は -128
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 上で示した例から
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // が成り立つ。232の8ビットにおける補数は -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Rust 1.45以降、浮動小数点数を整数にキャストするとき、
    // `as`キーワードが *飽和的キャスト* を行います。
    // 浮動小数点数の値が上限を超えたり下限を下回ったりする場合は、
    // 戻り値は越えられた境界の値となります。

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // この挙動は実行時にややコストがかかるため、安全でない方法で回避できます。
    // ただし、結果はオーバーフローしたり *不正確な値* を返す場合があります。
    // この方法は賢く使いましょう:
    unsafe {
        println!("hogehogehogehogehogehgoeghoegheogheohgoehogehogehoge");
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}