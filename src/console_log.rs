#[cfg(not(feature = "server"))]
pub fn console_log(message: &str) {
    let js = format!(
        r#"
        console.log('{}');
    "#,
        message
    );

    crate::eval(&js);
}

#[cfg(feature = "server")]
pub fn console_log(message: &str) {
    println!("{}", message);
}
