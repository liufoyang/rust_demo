fn main() {
    println!("Hello, world!");
    baseDateType();

    ownShip();

    baseOwnAndReference();

    mutOwnAndReference();
}

fn baseDateType() {
    let demoStr = 'a';

    let demoInt:u32 = 1024;
    let demoFloat = 10.0;
    let demoBoolean = true;

    println!("base type for char {}, int {}, float {}, boolean {}", demoStr,demoInt, demoFloat, demoBoolean );

    // let default 不变的，下面语句错误
    //demoInt = 120;

    // 数组的定义方法
    let demoArray = [1,2,3,4,5];
    println!("base array {}", demoArray[0]);

    // 可变的变量需要加mut声明
    let mut demoMutStr:String = String::from("hello");
    demoMutStr.push_str(", world");
    demoMutStr.push_str(", from rust");
    println!("mut varibality {}", demoMutStr);

}

fn ownShip() {
    let defaultOwn:String  = String::from("own");
    let defaultOwnInt:i32 = 32;
    println!("the own is ownMove {}", defaultOwn);

    // 所有者转移到新的，老的会失效
    let ownMove = defaultOwn;
    let ownMoveInt:i32 = defaultOwnInt;
    println!("the own is ownMove {}", ownMove);
    println!("the own is ownMoveInt {}", ownMoveInt);

   //println!("own have move {}", defaultOwn);   // 再使用原来的变量，就回报错，这里不能通过编译

    // 基础类型，放在stack，没有own的转移，直接stack的copy
    println!("own have move {}", defaultOwnInt);

    // own直接转移到函数内，最终范围结束被清理
    let passOwn:String = String::from("pass Own");
    takeOwnShip(passOwn);

    // 这里再使用，报错
   // println!("own have move {}", passOwn);

    // 基本类型，存放在stack，  own不传递，只复制值
    notTakeOwnShip(ownMoveInt);
    println!("the own is own not pass {}", ownMoveInt);
}

fn takeOwnShip(ownPass:String) {
    println!("own have move in fun {}", ownPass);
}

fn notTakeOwnShip(ownPass:i32) {
    println!("own have move in fun {}", ownPass);
}

fn baseOwnAndReference() {
    // 变量第一个持有，就是这变量的ownship
    let defaultOwn:String = String::from("dafualOwn");

    //
    let mut reference: &String = &defaultOwn;
    {
        // we only can use when ownship 在范围块内
        let ownScop:String = String::from("ownscop");

        reference = &ownScop;

        println!("in scop use {}", reference);
    }

    // reference 借来的引用， 因为owner范围块结束，这里使用会报错 borrowed value does not live long enough
    // println!("out scop use {}", reference);

}


fn mutOwnAndReference() {
    // 变量第一个持有，就是这变量的ownship
    let mut defaultOwn:String = String::from("dafualOwn");

    //
    let mut reference1: &String = &mut defaultOwn;

    // 借用只能借给一个，不能同时使用
    //let mut reference2: &String = &mut defaultOwn;
    //println!("in scop use {}", reference2);

    {
        // we only can use when ownship 在范围块内
        let ownScop: String = String::from("ownscop");

        reference1 = &ownScop;
        println!("in scop use {}", reference1);

        // 这里 借用可以， 因为refenc1已结结束借用了。
        let mut reference2 = &mut defaultOwn;
        println!("in scop use one borrow {}", reference2);

        // 把可变的变量借给第二个可变引用，则会报错，同时只能借给一个可变引用
        //reference1 = &mut defaultOwn;
        //println!("in scop use {} {}", reference1, reference2);
    }

    // 这里报错，因为reference1借的变量已结结束范围，被清理 borrow later used here
    //println!("in scop use {} {}", reference1, reference1);

}
