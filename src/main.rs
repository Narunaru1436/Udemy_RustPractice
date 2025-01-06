
// cargo runコマンドでコンパイルと実行が同時にできる

fn main() {
    /*
    println!("Hello, world!"); // 改行される
    print!("Hello, "); // 改行されない
    print!("world!");
    */

    let mut a:i32 = 100;
    //println!("{}", a);

    a = 200;
    //println!("{}", a);

    const MAX_COUNT : i32 = 1000;
    //println!("{}", MAX_COUNT);

    let x:i32 = 2;
    let y:f64 = 3.0;
    //let z:i32 = x + y; // 異なる型どうしは演算できない
    let z1:i32 = x + 3;
    let z2:f64 = y * 3.0;
    //println!("{}", z1);
    //println!("{}", z2);

    // キャスト
    let z3:f64 = z1 as f64 * 5.0;
    //print!("{}", z3);

    // タプルの宣言と初期化
    let t1: (i32, bool, f64) = (100, true, 9.0);
    //println!("{:?}", t1);

    let (x, y, _) = t1;
    //println!("{}", x);
    //println!("{}", y);
    //println!("{}", z);

    // 配列
    let list1:[i32 ; 3] = [10, 20, 30]; // 要素数の変更ができない→ベクターはできる
    //println!("{:?}", list1);

    // ベクタ
    let v1:Vec<i32> = vec![0; 10]; // vec![初期化要素, 要素数]
    //println!("{:?}", v1);
    // 可変ベクタ
    let mut v2: Vec<i32> = Vec::new();
    v2.push(100);
    //println!("{:?}", v2);

    // RustではNullが存在しない
    // 値がある場合はSomeを返し、ない場合はNoneを返す
    let x:Option<i32> = v2.pop();
    //println!("{:?}", x);

    // 文字型
    let c1:char = 'a';

    // 文字列型
    let s1:&str = "Rust"; // 変更不可
    // String型
    let s2:String = String::from("This is Rust"); 
    let s3:String = "Language".to_string();
    let mut s4:String = "New Language".to_string();

    // 変更可
    s4 = "Pass!".to_string();
    //println!("{}", s4);

    //FirstFunction();
    //println!("{}", Add(1, 20));

    // シャドーイング
    let n:i32 = 10;
    //println!("{}", n);
    {
        // スコープ
        let n:i32 = 20;
        //println!("{}", n);
    }
    //println!("{}", n);

    let x:i32 = 10;
    if x > 0 {
        println!("OK");
    }

    // Rustの基本概念として、式は返したり、変数に束縛したりできる

    // match→Swith文みたいなもの    
    let x:i32 = 0;
    match x {
        0 => println!("This is Zero"),
        1 => {
            println!("This is One");
            println!("This is One");
        },
        _=> println!("Other")
    };

    // loop → 無限ループ
    //loop {
        //println!("Hello");
    //}

    // while 
    let mut cnt:i32 = 0;
    //while cnt <= 10 {
      //  println!("{}", cnt);
        //cnt+=1;
    //}

    // for 
    for i in [10, 20, 30] {
        println!("{}", i);
    }




}


fn FirstFunction() {
    println!("Hello, My new World!");
}

fn Add(x:i32, y:i32) -> i32
{
    return x + y // ;なしで書くほうが一般的
}