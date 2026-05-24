use descriptive_unwrap::OptionUnwrapExt as _;

let mut option: Option<i32> = None;
let value = option.take().none_is_unreachable();
