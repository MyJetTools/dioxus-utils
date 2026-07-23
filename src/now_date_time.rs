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

#[cfg(feature = "server")]
pub fn now_local_date_time() -> DateTimeAsMicroseconds {
    use rust_extensions::chrono::Local;
    let now = Local::now();
    let unix_microseconds = now.naive_local().and_utc().timestamp_micros();
    DateTimeAsMicroseconds::new(unix_microseconds)
}

#[cfg(not(feature = "server"))]
pub fn now_local_date_time() -> DateTimeAsMicroseconds {
    let result = crate::eval("new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString()");

    let result = result.as_string();

    if result.is_none() {
        panic!("Failed getting local new Date().toISOString()");
    }

    let str = result.unwrap();

    let result = DateTimeAsMicroseconds::from_str(&str);

    if result.is_none() {
        panic!(
            "Failed parsing local new Date().toISOString(). String is {}",
            str
        );
    }

    result.unwrap()
}
