pub fn test() {
    let _a = String::from("abc");
    // 存储堆 -> 移动
    // let _b = _a; 不可访问

    // 克隆
    let _b = _a.clone() + "456";
    println!("{}", format!("{_a},{_b}"));

    // 存储栈 -> 拷贝
    let _c = ("a", "b");
    let _d = _c;
    println!("{}", format!("{_c:#?}"));

    // 传递引用
    let _e = String::from("hello, string");
    let len = get_length(&_e);
    println!("{_e}, {len}");

    // 修改
    let mut _f = String::from("hello");
    edit_str(&mut _f);

    // 可修改引用
    
    // err
    // let _g: &mut String = &mut _f;
    // let _f: &mut String = &mut _f;
    // println!("{_g}, {_f}");

    // ok
    let _g: &mut String = &mut _f;
    println!("{_g}");
    let _f: &mut String = &mut _f;
    println!("{_f}");

    
}

fn get_length(item: &String) -> usize {
    item.len()
}

fn edit_str(item: &mut String) -> usize {
    item.push_str(", push");
    item.len()
}
