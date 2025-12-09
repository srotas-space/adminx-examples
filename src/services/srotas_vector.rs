use actix_web::web;
use srotas_vector_sdk::ovm::{OvmExt, UpsertItem, Filter};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::srotas::initialize::SrotasVectorOvm;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DocMeta { doc: String, lang: String }

pub async fn example_handler(srotas: web::Data<Arc<SrotasVectorOvm>>) -> actix_web::HttpResponse {
    // typed collection: 4D DocMeta
    let coll = srotas.client.collection::<DocMeta, 4>("demo-typed");

    // ensure exists (no-op if already present)
    let _ = coll.create_if_absent(Some("typed demo".into())).await;

    // upsert
    let _id = coll.upsert_one(UpsertItem {
        id: None,
        embedding: [0.1,0.2,0.3,0.4],
        metadata: DocMeta { doc: "a".into(), lang: "en".into() }
    }).await.unwrap();

    // query
    let hits = coll.query([0.1,0.2,0.3,0.4], 5, Some(Filter::new().eq("lang", "en")))
        .await
        .unwrap();

    actix_web::HttpResponse::Ok().json(hits)
}
