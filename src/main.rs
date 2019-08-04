mod flycoin;

use flycoin::fly_coin;
fn main() {
    println!("Hello, world!");
    let mut s = "aaaa";
    s = "bbb";
    let mut b = s.to_string();
    b.push_str("cccc");

    base_date_type();

    own_ship();

    base_own_and_reference();

    mut_own_and_reference();

    //fly_coin
}

fn base_date_type() {
    let demo_str = 'a';

    let demo_int:u32 = 1024;
    let demo_float = 10.0;
    let demo_boolean = true;

    println!("base type for char {}, int {}, float {}, boolean {}", demo_str,demo_int, demo_float, demo_boolean );

    // let default 不变的，下面语句错误
    //demoInt = 120;

    // 数组的定义方法
    let demo_array = [1,2,3,4,5];
    println!("base array {}", demo_array[0]);

    // 可变的变量需要加mut声明
    let mut demo_mut_Str:String = String::from("hello");
    demo_mut_Str.push_str(", world");
    demo_mut_Str.push_str(", from rust");
    println!("mut varibality {}", demo_mut_Str);

}

fn own_ship() {
    let default_own:String  = String::from("own");
    let default_ownInt:i32 = 32;
    println!("the own is own_move {}", default_own);

    // 所有者转移到新的，老的会失效
    let own_move = default_own;
    let own_move_int:i32 = default_ownInt;
    println!("the own is own_move {}", own_move);
    println!("the own is own_move_int {}", own_move_int);

   //println!("own have move {}", default_own);   // 再使用原来的变量，就回报错，这里不能通过编译

    // 基础类型，放在stack，没有own的转移，直接stack的copy
    println!("own have move {}", default_ownInt);

    // own直接转移到函数内，最终范围结束被清理
    let pass_own:String = String::from("pass Own");
    take_own_ship(pass_own);

    // 这里再使用，报错
   // println!("own have move {}", pass_own);

    // 基本类型，存放在stack，  own不传递，只复制值
    not_take_own_ship(own_move_int);
    println!("the own is own not pass {}", own_move_int);
}

fn take_own_ship(own_pass:String) {
    println!("own have move in fun {}", own_pass);
}

fn not_take_own_ship(own_pass:i32) {
    println!("own have move in fun {}", own_pass);
}

fn base_own_and_reference() {
    // 变量第一个持有，就是这变量的ownship
    let default_own:String = String::from("dafualOwn");

    //
    let mut reference: &String = &default_own;
    {
        // we only can use when own_ship 在范围块内
        let own_scop:String = String::from("own_scop");

        reference = &own_scop;

        println!("in scop use {}", reference);
    }

    // reference 借来的引用， 因为owner范围块结束，这里使用会报错 borrowed value does not live long enough
    // println!("out scop use {}", reference);

}


fn mut_own_and_reference() {
    // 变量第一个持有，就是这变量的ownship
    let mut default_own:String = String::from("dafualOwn");

    //
    let mut reference1: &String = &mut default_own;

    // 借用只能借给一个，不能同时使用
    //let mut reference2: &String = &mut default_own;
    //println!("in scop use {}", reference2);

    {
        // we only can use when own_ship 在范围块内
        let own_scop: String = String::from("own_scop");

        reference1 = &own_scop;
        println!("in scop use {}", reference1);

        // 这里 借用可以， 因为refenc1已结结束借用了。
        let reference2 = &mut default_own;
        println!("in scop use one borrow {}", reference2);

        // 把可变的变量借给第二个可变引用，则会报错，同时只能借给一个可变引用
        //reference1 = &mut default_own;
        //println!("in scop use {} {}", reference1, reference2);
    }

    // 这里报错，因为reference1借的变量已结结束范围，被清理 borrow later used here
    //println!("in scop use {} {}", reference1, reference1);

}
