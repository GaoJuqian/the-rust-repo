pub fn test_1(_arr: [usize; 5], len: u8) {

    // 函数行参类型固定
    println!("{len}");

    // 默认类型
    let _a = 123;
    let _b = '1';
    let _c = "abcd";
    let _d = true;

    // 元组 类型不限 长度固定
    let _e = (1, "abc", 3, true);
    // 解构元组
    let (aaa, _, _, _) = _e;
    println!("{aaa}{}", _e.1);

    // 数组单一类型 长度固定
    let _d: [usize; 10] = [1, 2, 3, 45, 5, 7647645, 6456, 456, 456, 131231312321312];
}
