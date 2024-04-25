use axum::http::StatusCode;
use metrics::counter;

pub const DEFAULT_CACHE_CONTROL_HEADER_VALUE: &str =
    "public, max-age=300, s-maxage=300, stale-while-revalidate=300, stale-if-error=300";

pub const DEFAULT_TIMEOUT_IN_MILLI: u64 = 350;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("{}", err);

    let labels = [("error", format!("{}!", err))];

    counter!("request_error", &labels);

    return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
}
