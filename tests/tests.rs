use std::error::Error;
use std::env;
use hnews::html::client::Client;

#[test]
fn test_login() -> Result<(), Box<dyn Error>> {
    let user = env::var("HN_USER")?;
    let pwd = env::var("HN_PASS")?;
    println!("user = {:?}", user);
    println!("pwd = {:?}", pwd);
    let mut client = Client::new(&user, &pwd);
    client.login()?;

    Ok(())
}