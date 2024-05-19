#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);


#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle { top_left, bottom_right } = self;
        let width = bottom_right.x - top_left.x;
        let height = top_left.y - bottom_right.y;
        width * height
    }
    fn square(left_top_point: &Point, size: &f32) -> Rectangle {
        Rectangle {
            top_left: Point{x: left_top_point.x, y: left_top_point.y},
            bottom_right: Point{ x: left_top_point.x + size, y: left_top_point.y - size}
        }
    }
}

fn main() {
    // Person構造体：インスタンス生成
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    // ポイント構造体：インスタンス生成
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // ポイント構造体：フィールドの一部を変更
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // ポイント構造体：パターンマッチ
    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // ペア構造体：パターンマッチ
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // 長方形構造体：面積計算(左上(0, 10)と右下(10, 0)の座標から計算)
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    println!("Rectangle area: {}", rect.rect_area());

    let point = Point{x: 0.0, y: 0.0};
    let square = Rectangle::square(&point, &5.0);
    println!("Square: {:?}", square);
    println!("square area: {}", square.rect_area());

}
