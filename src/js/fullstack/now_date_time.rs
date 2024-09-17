use rust_extensions::date_time::DateTimeAsMicroseconds;

pub fn now_date_time() -> DateTimeAsMicroseconds {
    let result = super::eval("new Date().toISOString()");

    let result = result.as_string();

    if result.is_none() {
        panic!("Failed getting new Date().toISOString()");
    }

    let str = result.unwrap();

    let result = DateTimeAsMicroseconds::from_str(&str);

    if result.is_none() {
        panic!("Failed parsing new Date().toISOString(). String is {}", str);
    }

    result.unwrap()
}
