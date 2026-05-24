use descriptive_unwrap::none_is_unreachable;

let mut option: Option<i32> = None;
let value = none_is_unreachable(option.take());
