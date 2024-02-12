fn main() {
    println!("Hello, world!");

    // Chapter 01
    example_01_04();
    example_01_05();
    example_01_06();
    example_01_07();

    // Chapter 02
    example_02_01();
    example_02_02();
    example_02_03();
    example_02_04();
    example_02_05();
    example_02_06();
    example_02_10();
    example_02_11();
    example_02_13();
    example_02_14();
    example_02_15();
    example_02_16();

    // Chapter 03
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

// --------------------------------------------------------------------------------
// Chapter 01
// --------------------------------------------------------------------------------
fn example_01_04() {
    print!("{}, {}!", "Hello", "world");
}

fn example_01_05() {
    print!("First line\nSecond line\nThird line\n");
}

fn example_01_06() {
    println!("text of the line");
}

fn example_01_07() {
    print!("text of the line\n");
}

// --------------------------------------------------------------------------------
// Chapter 02
// --------------------------------------------------------------------------------
fn example_02_01() {
    println!("The sum is {}", 80 + 34);
}

fn example_02_02() {
    println!("{} + {} = {}", 80, 34, 80 + 34);
}

fn example_02_03() {
    println!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4));
}

fn example_02_04() {
    println!("The sum is {}", 80.3 + 34.8);
}

fn example_02_05() {
    println!("The sum is {}", 80.3 + 34.9);
}

fn example_02_06() {
    println!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
}

fn example_02_10() {
    print!("{} + ", 80);
    print!("{} = ", 34);
    print!("{}\n", 80 + 31);
}

fn example_02_11() {
    print!("{} + ", 80);print!("{} = ", 34);
            print     !       ("{}\n"  ,
        80        +    31 )   ;
}

fn example_02_13() {
    println!("{}", "These
        are
        three lines");
}

fn example_02_14() {
    println!("{}", "This \
        is \
        just one line");
}

fn example_02_15() {
    println!("{}", "These
are
three lines");
}

fn example_02_16() {
    println!("{}", "These \n\
                    are\n\
                    three lines");
}

// --------------------------------------------------------------------------------
// Chapter 03
// --------------------------------------------------------------------------------
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
