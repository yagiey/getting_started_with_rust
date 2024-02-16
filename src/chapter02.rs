// --------------------------------------------------------------------------------
// Chapter 02
// --------------------------------------------------------------------------------
pub fn chapter02() {
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
}

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
