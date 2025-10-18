use crate::model::RateError;

trait Redis {
    async fn incr(&self, key: &str, increment: i32) -> Result<i32, RateError<'_>>;
    async fn expire(&self, key: &str, ttl: i32 ) -> Result<i32, RateError<'_>>;
}