use actix_web::Error;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use super::schema::UserProfile;

pub async fn get_profile(client: &Client, uuid: Uuid) -> Result<UserProfile, Error> {
    let stmt = include_str!("../sql_queries/get_profile.pgsql");
    let stmt = stmt.replace("$table_fields", &UserProfile::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[&uuid])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| UserProfile::from_row_ref(row).unwrap())
        .next();

    Ok(results.unwrap())
}

pub async fn login_profile(client: &Client, user_info: UserProfile) -> Result<UserProfile, Error> {
    let stmt = include_str!("../sql_queries/get_by_username.pgsql");
    let stmt = stmt.replace("$table_fields", &UserProfile::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[&user_info.username])
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| UserProfile::from_row_ref(row).unwrap())
        .next()
        .unwrap();

    if results.password != user_info.password {
        panic!("Not yet implemented");
    }

    Ok(results)
}

pub async fn add_user(client: &Client, user_info: UserProfile) -> Result<UserProfile, Error> {
    let _stmt = include_str!("../sql_queries/add_user_profile.pgsql");
    let _stmt = _stmt.replace("$table_fields", &UserProfile::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&user_info));

    let profiles = client
        .query(
            &stmt,
            &[
                &user_info.first_name,
                &user_info.last_name,
                &user_info.email,
                &user_info.username,
                &user_info.password,
                &user_info.phone_number,
                &user_info.birth_date,
            ],
        )
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| UserProfile::from_row_ref(row).unwrap())
        .next();
    assert!(profiles.is_some());
    Ok(profiles.unwrap())
}

pub async fn update_user(
    client: &Client,
    user_info: UserProfile,
    uuid: Uuid,
) -> Result<UserProfile, Error> {
    let _stmt = include_str!("../sql_queries/update_profile.pgsql");
    let _stmt = _stmt.replace("$table_fields", &UserProfile::sql_table_fields());
    println!("Stmt: {:?}", _stmt);
    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("Prepared values: {:?}", serde_json::to_string(&user_info));

    let profiles = client
        .query(
            &stmt,
            &[
                &uuid,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.email,
                &user_info.phone_number,
                &user_info.birth_date,
            ],
        )
        .await
        .unwrap()
        .iter()
        .take(1)
        .map(|row| UserProfile::from_row_ref(row).unwrap())
        .next();
    assert!(profiles.is_some());
    Ok(profiles.unwrap())
}
