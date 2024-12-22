use crate::entity::prelude::Tickets;
use crate::entity::tickets;
use crate::service::util;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::{ActiveValue, QuerySelect};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize)]
struct Ticket {
    pub id: i32,
    pub code_num: String,
    pub create_at: Option<DateTime>,
    pub update_at: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub code_num: String,
}

pub async fn create_tickets(
    State(state): State<AppState>,
    Json(create_ticket): Json<CreateTicket>,
) -> impl IntoResponse {
    info!("create_ticket, {:?}", create_ticket);
    let ticket = tickets::ActiveModel {
        code_num: ActiveValue::Set(create_ticket.code_num),
        ..Default::default()
    };
    let _ = ticket.insert(&state.db).await;
    "Tickets created!"
}

#[derive(Serialize, Deserialize)]
pub struct ListQuery {
    page: Option<u64>,
    size: Option<u64>,
}

pub async fn list_tickets(
    State(state): State<AppState>,
    Query(list_query): Query<ListQuery>,
) -> impl IntoResponse {
    info!(
        "Listing tickets page is {:?}, size is {:?}",
        list_query.page, list_query.size
    );
    let page = list_query.page.unwrap_or(1);
    let size = list_query.size.unwrap_or(10);
    let offset = (page - 1) * size;

    let tickets = Tickets::find()
        .order_by_asc(tickets::Column::Id)
        .offset(offset)
        .limit(size);
    let tickets = tickets.all(&state.db).await.expect("Error");
    info!("tickets: {:?}", tickets);

    let some_data: Option<Vec<Ticket>> = Some(
        tickets
            .into_iter()
            .map(|ticket| Ticket {
                id: ticket.id,
                code_num: ticket.code_num,
                create_at: ticket.create_at,
                update_at: ticket.update_at,
            })
            .collect(),
    );

    util::resp_success(some_data)
}

pub async fn get_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<i32>,
) -> impl IntoResponse {
    info!("Getting ticket...{:?}", ticket_id);
    let ticket: Option<tickets::Model> = Tickets::find_by_id(ticket_id)
        .one(&state.db)
        .await
        .expect("Error");
    info!("ticket: {:?}", ticket);

    if let Some(ticket) = ticket {
        let some_data:Option<Ticket> = Some(Ticket {
            id: ticket.id,
            code_num: ticket.code_num,
            create_at: ticket.create_at,
            update_at: ticket.update_at,
        });
        return util::resp_success(some_data);
    }
    util::resp_error(1, "Ticket not found".to_string())
}