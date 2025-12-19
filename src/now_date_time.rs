use rust_extensions::date_time::DateTimeAsMicroseconds;
#[cfg(feature = "server")]
pub fn now_date_time() -> DateTimeAsMicroseconds {
    DateTimeAsMicroseconds::now()
}

#[cfg(not(feature = "server"))]
pub fn now_date_time() -> DateTimeAsMicroseconds {
    let result = crate::eval("new Date().toISOString()");

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
