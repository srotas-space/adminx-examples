//// Users/xsm/Documents/workspace/XSM/crates/test/src/srotas/service.rs
use actix_web::{web, HttpResponse, Responder, http::StatusCode as ActixStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use srotas_vector_sdk::ovm::{Filter, OvmExt, UpsertItem};
use srotas_vector_sdk::{SdkError};
use crate::srotas::initialize::SrotasVectorOvm;



#[derive(serde::Serialize)]
struct HitOut {
    id: uuid::Uuid,
    score: f32,
    metadata: DocMeta,
}

// ---- typed metadata & constants ----
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocMeta {
    pub doc: String,
    pub lang: String,
    pub views: Option<u64>,
    pub tags: Option<Vec<String>>,
}

const DIM: usize = 4; // compile-time embedding dimension

// ---- DTOs ----
#[derive(Deserialize)]
pub struct CreateCollectionBody {
    pub description: Option<String>,
    pub dim: Option<usize>, // ignored; DIM is compile-time
}

#[derive(Deserialize, Clone)]
pub struct UpsertBody {
    pub id: Option<Uuid>,
    pub embedding: [f32; DIM],
    pub metadata: DocMeta,
}

#[derive(Deserialize)]
pub struct BatchUpsertBody {
    pub items: Vec<UpsertBody>,
}

#[derive(Deserialize)]
pub struct QueryBody {
    pub embedding: [f32; DIM],
    pub top_k: usize,
    pub filter_lang_in: Option<Vec<String>>,
    pub filter_eq: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Deserialize)]
pub struct Paging {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// Small helper: SdkError → HttpResponse
fn respond_err(e: SdkError) -> HttpResponse {
    match e {
        SdkError::Http { status, body } => {
            let sc: ActixStatus = ActixStatus::from_u16(status.as_u16())
                .unwrap_or(ActixStatus::INTERNAL_SERVER_ERROR);
            HttpResponse::build(sc).body(body)
        }
        other => {
            // fallback for other error kinds
            HttpResponse::InternalServerError().body(format!("{:?}", other))
        }
    }
}

// ---- router ----
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // collection-level
        .route("/collections/{name}", web::post().to(create_collection))
        .route("/collections/{name}", web::delete().to(drop_collection))
        .route("/collections/{name}/count", web::get().to(count))
        .route("/collections/{name}/ids", web::get().to(list_ids))
        // UPSERT (idempotent)
        .route("/collections/{name}/docs", web::post().to(upsert_one))
        .route("/collections/{name}/docs/batch", web::post().to(upsert_batch))
        // STRICT INSERT / UPDATE
        .route("/collections/{name}/docs/insert", web::post().to(insert_one))
        .route("/collections/{name}/docs/batch/insert", web::post().to(insert_batch))
        .route("/collections/{name}/docs/update", web::post().to(update_one))
        .route("/collections/{name}/docs/batch/update", web::post().to(update_batch))
        // direct by id
        .route("/docs/{id}", web::get().to(get_by_id))
        .route("/docs/{id}", web::delete().to(delete_by_id));
}

// ---- handlers ----

async fn create_collection(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<CreateCollectionBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    match coll.create_if_absent(body.description.clone()).await {
        Ok(meta) => HttpResponse::Ok().json(meta),
        Err(e) => respond_err(e),
    }
}

async fn drop_collection(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    match coll.drop().await {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => respond_err(e),
    }
}

async fn count(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    match coll.count().await {
        Ok(n) => HttpResponse::Ok().json(n),
        Err(e) => respond_err(e),
    }
}

async fn list_ids(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    q: web::Query<Paging>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    let offset = q.offset.unwrap_or(0);
    let limit = q.limit.unwrap_or(100).min(1000);
    match coll.list_ids(offset, limit).await {
        Ok(ids) => HttpResponse::Ok().json(ids),
        Err(e) => respond_err(e),
    }
}

async fn upsert_one(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<UpsertBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    let item = UpsertItem { id: body.id, embedding: body.embedding, metadata: body.metadata.clone() };
    match coll.upsert_one(item).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => respond_err(e),
    }
}

async fn upsert_batch(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<BatchUpsertBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    if body.items.is_empty() {
        return HttpResponse::BadRequest().body("no items provided");
    }
    let items = body.items.iter().cloned().map(|it| UpsertItem {
        id: it.id, embedding: it.embedding, metadata: it.metadata,
    }).collect::<Vec<_>>();
    match coll.upsert_many(items).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => respond_err(e),
    }
}

// ========== STRICT INSERT / UPDATE ==========

async fn insert_one(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<UpsertBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    let item = UpsertItem { id: body.id, embedding: body.embedding, metadata: body.metadata.clone() };
    match coll.insert_one(item).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => respond_err(e), // Will be 409 if id exists
    }
}

async fn insert_batch(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<BatchUpsertBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    if body.items.is_empty() {
        return HttpResponse::BadRequest().body("no items provided");
    }
    let items = body.items.iter().cloned().map(|it| UpsertItem {
        id: it.id, embedding: it.embedding, metadata: it.metadata,
    }).collect::<Vec<_>>();
    match coll.insert_many(items).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => respond_err(e), // Will be 409 if any id exists
    }
}

async fn update_one(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<UpsertBody>,
) -> impl Responder {
    if body.id.is_none() {
        return HttpResponse::BadRequest().body("id required for update");
    }
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    let item = UpsertItem { id: body.id, embedding: body.embedding, metadata: body.metadata.clone() };
    match coll.update_one(item).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => respond_err(e), // 404 if id not found
    }
}

async fn update_batch(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<BatchUpsertBody>,
) -> impl Responder {
    if body.items.iter().any(|it| it.id.is_none()) {
        return HttpResponse::BadRequest().body("id required for each item in update batch");
    }
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    if let Err(e) = coll.create_if_absent(None).await {
        return respond_err(e);
    }
    let items = body.items.iter().cloned().map(|it| UpsertItem {
        id: it.id, embedding: it.embedding, metadata: it.metadata,
    }).collect::<Vec<_>>();
    match coll.update_many(items).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => respond_err(e), // 404 if any id not found
    }
}

// ========== Query / Get / Delete ==========

async fn query(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    name: web::Path<String>,
    body: web::Json<QueryBody>,
) -> impl Responder {
    let coll = srotas.client.collection::<DocMeta, DIM>(&name);
    // Build a filter if provided
    let mut filter = None;
    if body.filter_lang_in.is_some() || body.filter_eq.is_some() {
        let mut f = Filter::new();
        if let Some(list) = &body.filter_lang_in { f = f.in_list("lang", list.clone()); }
        if let Some(eqmap) = &body.filter_eq {
            for (k, v) in eqmap { f = f.eq(k.clone(), v.clone()); }
        }
        filter = Some(f);
    }
    match coll.query(body.embedding, body.top_k, filter).await {
        Ok(hits) => {
            let out: Vec<HitOut> = hits.into_iter()
                .map(|h| HitOut { id: h.id, score: h.score, metadata: h.metadata })
                .collect();
            HttpResponse::Ok().json(out)
        }
        Err(e) => respond_err(e),
    }
}

async fn get_by_id(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match srotas.client.get_vector(*id).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => respond_err(e),
    }
}

async fn delete_by_id(
    srotas: web::Data<Arc<SrotasVectorOvm>>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match srotas.client.delete_vector(*id).await {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(e) => respond_err(e),
    }
}
