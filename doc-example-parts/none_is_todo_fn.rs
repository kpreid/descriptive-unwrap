use descriptive_unwrap::none_is_todo;

fn main() {
    let mut option: Option<i32> = None;
    let value = none_is_todo(option.take());
}
