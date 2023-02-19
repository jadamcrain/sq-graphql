use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "all_films.graphql",
    response_derives = "Debug"
)]
pub struct Query;

const URL: &str = "https://swapi-graphql.netlify.app/.netlify/functions/index";

async fn get_all_films(variables: query::Variables) -> Result<(), Box<dyn Error>> {
    // this is the important line
    let request_body = Query::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post(URL).json(&request_body).send().await?;
    let response_body: Response<query::ResponseData> = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    get_all_films(query::Variables).await?;
    Ok(())
}
