use chrono;
use tokio;
use futures::Future;
use std::pin::Pin;
use std::time::Duration;


pub async fn time_it<T, F>(f: F) -> (T, chrono::Duration)
where F: Fn() -> Pin<Box<dyn Future<Output = T>>>,
{
    let t0 = chrono::Utc::now();
    let res = f().await;
    let t1 = chrono::Utc::now();
    let delta = t1 - t0;
    (res, delta)
}
