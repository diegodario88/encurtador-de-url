use std::u32;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;
use sqlx::PgPool;
use url::Url;

use crate::utils::{internal_error, DEFAULT_CACHE_CONTROL_HEADER_VALUE, DEFAULT_TIMEOUT_IN_MILLI};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: String,
    pub target_url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkTarget {
    pub target_url: String,
}

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    pub status: String,
    pub info: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountedLinkStatistic {
    pub amount: Option<i64>,
    pub referer: Option<String>,
    pub user_agent: Option<String>,
}

impl HealthResponse {
    fn healthy() -> HealthResponse {
        return HealthResponse {
            status: String::from("ok"),
            info: String::from("Todos os serviços estão saudáveis"),
        };
    }
}

fn generate_id() -> String {
    let random_number = rand::thread_rng().gen_range(0..u32::MAX);
    let random_url = general_purpose::URL_SAFE_NO_PAD.encode(random_number.to_string());

    return random_url;
}

pub async fn health_check() -> impl IntoResponse {
    return (StatusCode::OK, Json(HealthResponse::healthy()));
}

pub async fn create_link(
    State(pool): State<PgPool>,
    Json(new_link): Json<LinkTarget>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let insert_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let url = Url::parse(&new_link.target_url)
        .map_err(|_| (StatusCode::CONFLICT, "URL mal formatada".into()))?
        .to_string();

    let new_link_id = generate_id();

    let query = sqlx::query_as!(
        Link,
        r#"
        with inserted_link as (
            insert into links(id, target_url)
            values ($1, $2)
            returning id, target_url
        )
        select id, target_url from inserted_link
        "#,
        &new_link_id,
        &url,
    )
    .fetch_one(&pool);

    let new_link = tokio::time::timeout(insert_timeout, query)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    tracing::debug!(
        "Criado novo link com o identificador {} apontando para {}",
        new_link.id,
        new_link.target_url
    );

    return Ok(Json(new_link));
}

pub async fn update_link(
    State(pool): State<PgPool>,
    Path(link_id): Path<String>,
    Json(update_link): Json<LinkTarget>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let update_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let url = Url::parse(&update_link.target_url)
        .map_err(|_| (StatusCode::CONFLICT, "URL mal formatada".into()))?
        .to_string();

    let query = sqlx::query_as!(
        Link,
        r#"
        with updated_link as (
            update links set target_url = $1 where id = $2
            returning id, target_url
        )
        select id, target_url from updated_link
        "#,
        &url,
        &link_id
    )
    .fetch_one(&pool);

    let updated_link = tokio::time::timeout(update_timeout, query)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    tracing::debug!(
        "Link com id {} atualizado, agora apontanto para {}",
        updated_link.id,
        updated_link.target_url
    );

    return Ok(Json(updated_link));
}

pub async fn redirect(
    State(pool): State<PgPool>,
    Path(requested_link): Path<String>,
    headers: HeaderMap,
) -> Result<Response, (StatusCode, String)> {
    let select_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let query = sqlx::query_as!(
        Link,
        "select id, target_url from links where id = $1",
        requested_link
    )
    .fetch_optional(&pool);

    let link = tokio::time::timeout(select_timeout, query)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?
        .ok_or_else(|| "Não foi encontrado resultados".to_string())
        .map_err(|err| (StatusCode::NOT_FOUND, err))?;

    tracing::debug!(
        "Redirecionando o link {} para {}",
        requested_link,
        link.target_url
    );

    let referer_header = headers
        .get("referer")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or("Não informado");

    let user_agent = headers
        .get("user-agent")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or("Não informado");

    let insert_statistics_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let statistics_query = sqlx::query(
        r#"
            insert into link_statistics(link_id, referer, user_agent)
            values($1, $2, $3)
        "#,
    )
    .bind(&requested_link)
    .bind(&referer_header)
    .bind(&user_agent)
    .execute(&pool);

    let insert_statistics_result =
        tokio::time::timeout(insert_statistics_timeout, statistics_query).await;

    match insert_statistics_result {
        Err(elapsed) => tracing::error!(
            "Persistir as estatísticas resultou em erro. Ultrapassou o tempo limite de {} ms",
            elapsed
        ),
        Ok(Err(err)) => tracing::error!(
            "Persistir as estatísticas resultou no seguinte erro: {}",
            err
        ),
        _ => tracing::debug!(
            "Estatísticas salvas com sucesso para o id {}, referer {}, e user-agent {}",
            requested_link,
            referer_header,
            user_agent,
        ),
    }

    return Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", link.target_url)
        .header("Cache-Control", DEFAULT_CACHE_CONTROL_HEADER_VALUE)
        .body(Body::empty())
        .expect("Essa response deve ser contruída sempre"));
}

pub async fn get_link_statistics(
    State(pool): State<PgPool>,
    Path(link_id): Path<String>,
) -> Result<Json<Vec<CountedLinkStatistic>>, (StatusCode, String)> {
    let select_statistics_timeout = tokio::time::Duration::from_millis(DEFAULT_TIMEOUT_IN_MILLI);

    let query = sqlx::query_as!(
        CountedLinkStatistic,
        r#"
            select count(*) as amount, referer, user_agent from link_statistics
            group by link_id, referer, user_agent
            having link_id = $1
        "#,
        &link_id
    )
    .fetch_all(&pool);

    let counted_link_statistics = tokio::time::timeout(select_statistics_timeout, query)
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    tracing::debug!(
        "Estatísticas para o id: {} consultadas com sucesso",
        link_id
    );

    return Ok(Json(counted_link_statistics));
}
