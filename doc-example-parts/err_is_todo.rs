use descriptive_unwrap::ResultUnwrapExt as _;

let input = [0xB8, 0x65, 0x6c, 0x6c, 0x6f];
let string = std::str::from_utf8(&input).err_is_todo();
println!("{string}");
