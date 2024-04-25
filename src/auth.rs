use crate::utils::{internal_error, DEFAULT_TIMEOUT_IN_MILLI};

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use metrics::counter;
use sha3::{Digest, Sha3_256};
use sqlx::PgPool;

struct Settings {
    #[allow(dead_code)]
    id: String,
    encrypted_global_api_key: String,
}

pub async fn guard(
    State(pool): State<PgPool>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let labels = [("uri", format!("{}!", req.uri()))];

    let api_key = req.headers()
        .get("x-api-key")
        .map(|v| v.to_str().unwrap_or_default())
        .ok_or_else(|| {
            tracing::error!("Request não autorizada. Não foi possível encontrar o cabeçalho contendo a chave de autenticação");
            counter!("unauthenticated_calls_count", &labels);

            return (StatusCode::UNAUTHORIZED, "Unauthorized".into());
        })?;

    let select_settings_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let query = sqlx::query_as!(
        Settings,
        "select id, encrypted_global_api_key from settings where id = $1",
        "DEFAULT_SETTINGS"
    )
    .fetch_one(&pool);

    let select_settings_result = tokio::time::timeout(select_settings_timeout, query)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let mut hasher = Sha3_256::new();
    hasher.update(api_key.as_bytes());

    let provided_api_key = hasher.finalize();

    if select_settings_result.encrypted_global_api_key != format!("{provided_api_key:x}") {
        tracing::error!("Request não autorizada. A chave de autenticação provida no cabeçalho não corresponde com a armazenada");
        counter!("unauthenticated_calls_count", &labels);

        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }

    Ok(next.run(req).await)
}
