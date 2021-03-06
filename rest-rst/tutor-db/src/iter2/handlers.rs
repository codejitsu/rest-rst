use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;

    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<usize>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn get_course_details(app_state: web::Data<AppState>, params: web::Path<(usize, usize)>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::{NaiveDateTime, NaiveDate};
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("TUTOR_DATABASE_URL").expect("TUTOR_DATABASE_URL is not set in .env file");
        let db_pool = PgPool::new(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: String::from("All good"),
            visit_count: Mutex::new(0),
            db: db_pool
        });
        
        let tutor_id: web::Path<usize> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        dotenv().ok();

        let database_url = env::var("TUTOR_DATABASE_URL").expect("TUTOR_DATABASE_URL is not set in .env file");
        let db_pool = PgPool::new(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: String::from("All good"),
            visit_count: Mutex::new(0),
            db: db_pool
        });
        
        let params: web::Path<(usize, usize)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();

        let database_url = env::var("TUTOR_DATABASE_URL").expect("TUTOR_DATABASE_URL is not set in .env file");
        let db_pool = PgPool::new(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: String::from("All good"),
            visit_count: Mutex::new(0),
            db: db_pool
        });
        
        let new_course = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "The next course".into(),
            posted_time: Some(NaiveDate::from_ymd(2021, 2, 19).and_hms(21, 14, 51))
        };

        let param = web::Json(new_course);
        let resp = post_new_course(param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}