pub fn test_1(_arr: [usize; 5], len: u8) -> &'static str {
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

    // if
    let _e = if 1 == 1 { true } else { false };

    // loop
    let mut count = 0;
    let result = 'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up true;
            }
            remaining -= 1;
        }

        count += 1;
    };

    if result {
        println!("count = {}", count);
    }

    // for
    for number in (1..4).rev() {
        println!("{number}!");
    }

    // 函数的返回值等同于函数体最后一个表达式的值
    "ok"
}
