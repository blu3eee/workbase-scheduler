use axum::{ Extension, Json };

use crate::{
    appstate::{ AppState, AppResult },
    models::{ ResponseDataList, user::User },
    utilities::app_error::AppError,
    queries::user::UserQueries,
    prototypes::basic_queries::BasicQueries,
};

pub async fn get_all_user_handler(Extension(
    state,
): Extension<AppState>) -> AppResult<Json<ResponseDataList<User>>> {
    let mut conn = state.db_pool
        .get_conn()
        .map_err(|_| AppError::bad_request("cannot connect to database"))?;
    let users = UserQueries::find_all(&mut conn)?;

    Ok(Json(ResponseDataList { data: users }))
}
