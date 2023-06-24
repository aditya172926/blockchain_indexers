#[allow(unused)]

use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};

#[derive(Debug,FromRow)]
pub struct Event{
    id:i64,
    from:String,
    to:String,              //EOA or Contract address
    block_number:i32,
}

#[tokio::main]
async fn main() -> Result<(),sqlx::Error>{

//Create instance
    let pool=PgPoolOptions::new().max_connections(5).connect("postgres://postgres:[#password]@localhost/test").await?;

//Create table
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS event (
        id bigserial,
        "from" text,
        "to" text,
        block_number int   
        );"#,
        )
        .execute(&pool)
        .await?;

//insert new event


    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO event ("from", "to", block_number)
        VALUES ($1, $2, $3)
        RETURNING id
        "#
    )
    .bind("0xbfd232cebe066d048bdd042d285cc7924171323f")
    .bind("0x00000000000c2e074ec69a0dfb2997ba6c7d2e1e")
    .bind(9171138)
    .fetch_one(&pool)
    .await?;

//Fetch data
    
    let select_query = sqlx::query_as::<_, Event>(
        r#"
        SELECT id, "from", "to", block_number
        FROM event
        "#
    );

	let events: Vec<Event> = select_query.fetch_all(&pool).await?;
	println!("\n=== select events with query.map...: \n{:?}", events);

    Ok(())
}


// cargo watch -q -c -x run

