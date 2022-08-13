use rocket::local::blocking::Client;
use crate::{setup};
use crate::jwt::{DraftUser, Login};

#[test]
fn test_login() {
    dotenv::dotenv().unwrap();

    let user = DraftUser {
        email: "thomas@dooms.eu",
        password: "appeltaart",
    };

    let login = Login {
        email: "thomas@dooms.eu",
        password: "appeltaart",
    };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let rocket = runtime.block_on(setup(rocket::Config::default())).unwrap();

    let client = Client::tracked(rocket).expect("valid rocket instance");

    let request = client.post("/signup").json(&user);
    println!("{:?}", request.dispatch());

    let request = client.post("/login").json(&login);
    let response = request.dispatch();
    println!("{:?}", response.status());
    println!("{:?}", response.into_string().unwrap())
}