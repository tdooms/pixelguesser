use crate::routes::Credentials;
use crate::setup;
use rocket::local::blocking::Client;

#[test]
fn test_login() {
    dotenv::dotenv().unwrap();

    let credentials = Credentials { email: "thomas@dooms.eu", password: "appeltaart" };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let rocket = runtime.block_on(setup(rocket::Config::default())).unwrap();

    let client = Client::tracked(rocket).expect("valid rocket instance");

    let request = client.post("/signup").json(&credentials);
    println!("{:?}", request.dispatch());

    let request = client.post("/login").json(&credentials);
    let response = request.dispatch();
    println!("{:?}", response.status());
    println!("{:?}", response.into_string().unwrap())
}
