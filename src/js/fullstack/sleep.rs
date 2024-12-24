use std::time::Duration;

#[cfg(not(feature = "server"))]
pub async fn sleep(duration: Duration) {
    let millis = duration.as_millis();
    gloo_timers::future::TimeoutFuture::new(millis as u32).await;
}

#[cfg(feature = "server")]
pub async fn sleep(_duration: Duration) {}
