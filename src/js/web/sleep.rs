use std::time::Duration;

pub async fn sleep(duration: Duration) {
    let millis = duration.as_millis();
    gloo_timers::future::TimeoutFuture::new(millis as u32).await;
}
