pub fn test(){
    let  _a = String::from("abc");
    // 存储堆 -> 移动
    // let _b = _a; 不可访问
    
    // 克隆
    let  _b = _a.clone() + "456";
    println!("{}", format!("{_a},{_b}"));

    // (实现copy trait) 存储栈 -> 拷贝
    let _c = ("a", "b");
    let _d = _c;
    println!("{}", format!("{_c:#?}"));

}