let input = [0xB8, 0x65, 0x6c, 0x6c, 0x6f];

let string = err_is_todo(std::str::from_utf8(&input));
println!("{string}");
