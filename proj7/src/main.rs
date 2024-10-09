struct MyStruct {
    is_struct: bool,
    not_struct: bool
}


fn main() {
    println!("Hello, world!");
    let user1: MyStruct = MyStruct {
        is_struct: true,
        not_struct: false
    };
    let user2: MyStruct = MyStruct {
        is_struct: false,
        ..user1
    };
    println!("Struct1 = MyStruct = {}",user1.is_struct);
    println!("Struct2 = MyStruct = {}",user2.is_struct);
}
