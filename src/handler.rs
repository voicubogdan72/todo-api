use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;

use crate::{
    model::ToDoModel,
    schema::{CreateToDoSchema, FilterOptions, UpdateToDoSchmea},
    AppState,
};

pub async fn sarcini_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ToDoModel,
        "SELECT * FROM sarcini ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error getint the task"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let sarcini = query_result.unwrap();
    let json_response = serde_json::json!({
        "status": "succes",
        "result": sarcini.len(),
        "sarcini": sarcini
    });
    Ok(Json(json_response))
}

pub async fn create_sarcina_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateToDoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ToDoModel,
        "INSERT INTO sarcini (nume_sarcina, notita_sarcina, ora_sarcina, data_sarcina) VALUES ($1, $2, $3, $4) RETURNING *",
        body.nume_sarcina.to_string(),
        body.notita_sarcina.to_string(),
        body.ora_sarcina.to_string(),
        body.data_sarcina.to_string()
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(sarcina) => {
            let sarcina_response = json!({
                "status": "success", "data": json!({
                    "sarcina": sarcina
                })
            });
            return Ok((StatusCode::CREATED, Json(sarcina_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_sarcina_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ToDoModel, "SELECT * from sarcini WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;
    match query_result {
        Ok(sarcina) => {
            let sarcina_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "sarcina": sarcina
            })});
            return Ok(Json(sarcina_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Sarcina with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

// pub async fn edit_sarcina_handler(
//     Path(id): Path<uuid::Uuid>,
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<UpdateToDoSchmea>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let query_result = sqlx::query_as!(ToDoModel, "SELECT * FROM sarcina WHERE id = $1", id)
//         .fetch_one(&data.db)
//         .await;

//     if query_result.is_err() {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Sarcina with ID: {} not found", id)
//         });
//         return Err((StatusCode::NOT_FOUND, Json(error_response)));
//     }

//     let now = chrono::Utc::now();
//     let sarcina = query_result.unwrap();

//     let query_result = sqlx::query_as!(
//         ToDoModel,
//         "UPDATE sarcini SET nume_sarcina = $1, notita_sarcina = $2, ora_sarcina = $3, data_sarcina = $4, updated_at = $5 WHERE id = $6 RETURNING *",
//         body.nume_sarcina.to_owned().unwrap_or(sarcina.nume_sarcina),
//         body.notita_sarcina.to_owned().unwrap_or(sarcina.notita_sarcina),
//         body.ora_sarcina.to_owned().unwrap_or(sarcina.ora_sarcina.unwrap()),
//         body.data_sarcina.to_owned().unwrap_or(sarcina.data_sarcina.unwrap()),
//         now,
//         id
//     )
//     .fetch_one(&data.db)
//     .await
//     ;

//     match query_result {
//         Ok(sarcina) => {
//             let sarcina_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "sarcina": sarcina
//             })});

//             return Ok(Json(sarcina_response));
//         }
//         Err(err) => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"status": "error","message": format!("{:?}", err)})),
//             ));
//         }
//     }
// }

pub async fn delete_sarcina_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM sarcini  WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Sarcina with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "To do API by Youcu";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
