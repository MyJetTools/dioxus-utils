pub fn console_log(message: &str) {
    let js = format!(
        r#"
        console.log('{}');
    "#,
        message
    );

    super::eval(&js);
}
