#[cfg(not(feature = "server"))]
pub fn generate_uuid() -> String {
    let js = format!(r#"crypto.randomUUID()"#,);

    let result = crate::eval(&js);
    result.as_string().unwrap()
}

#[cfg(feature = "server")]
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
