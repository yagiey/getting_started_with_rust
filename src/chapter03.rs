// --------------------------------------------------------------------------------
// Chapter 03
// --------------------------------------------------------------------------------
pub fn chapter03() {
    example_03_01();
    example_03_02();
    example_03_03();
    example_03_06();
    example_03_07();
    example_03_09();
    example_03_12();
    example_03_16();
    example_03_17();
    example_03_18();
    example_03_19();
    example_03_20();
    example_03_21();
    example_03_22();
    example_03_27();
    example_03_29();
    example_03_30();
    example_03_31();
    example_03_32();
}

fn example_03_01() {
    let number = 12;
    let another_number = 53;
    println!("{}", number + another_number);
}

fn example_03_02() {
    let number = 12;
    println!("{} {}", number, 47);
}

fn example_03_03() {
    let mut number = 12;
    print!("{} ", number);
    number = 53;
    println!("{}", number);
}

fn example_03_06() {
    //#[allow(unused_mut)]
    let mut number = 12;
    println!("{}", number);
}

fn example_03_07() {
    let number;
    number = 12;
    println!("{}", number);
}

fn example_03_09() {
    let number1;
    let number2 = 22;
    number1 = number2;
    println!("{}", number1);
}

fn example_03_12() {
    let _number1 = 12;
}

fn example_03_16() {
    let truth = true;
    let falsity = false;
    println!("{} {}", truth, falsity);
}

fn example_03_17() {
    let truth = 5 > 2;
    let falsity = -12.3 >= 10.;
    println!("{} {} {}", truth, falsity, -50 < 6);
}

fn example_03_18() {
    println!("{} {} {}", "abc" < "abcd", "ab" < "ac", "A" < "a");
}

fn example_03_19() {
    let truth = true;
    let falsity = false;
    println!("{} {}", ! truth, ! falsity);
    println!("{} {} {} {}", falsity && falsity, falsity && truth, truth && falsity, truth && truth);
    println!("{} {} {} {}", falsity || falsity, falsity || truth, truth || falsity, truth || truth);
}

fn example_03_20() {
    println!("{}", true || true && !true);
}

fn example_03_21() {
    println!("{}", (true || true) && !true);
}

fn example_03_22() {
    let mut n = 1;
    print!("{} ", n);
    n = 2;
    print!("{} ", n);
    n = 3;
    //n = 3.14;
    println!("{}", n);
}

fn example_03_27() {
    let mut n = 1;
    print!("{} ", n);
    n = 2;
    print!("{} ", n);
    let n = 3.14;
    println!("{}", n);
}

fn example_03_29() {
    let x = 120; print!("{} ", x);
    let x = "abcd"; print!("{} ", x);
    let mut x = true; print!("{} ", x);
    x = false; println!("{}", x);
}

fn example_03_30() {
    let mut a = 12;
    a = a + 1;
    a = a - 4;
    a = a * 7;
    a = a / 6;
    println!("{}", a);
}

fn example_03_31() {
    let mut a = 12;
    a += 1;
    a -= 4;
    a *= 7;
    a /= 6;
    println!("{}", a);
}

fn example_03_32() {
    println!("{} {}", str::len("abcde"), "abcde".len());
}
