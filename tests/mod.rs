#[test]
#[cfg_attr(not(feature = "blocking"), ignore)]
fn test_blocking() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    let now = Instant::now();
    let data = comcigan::blocking::request_target()?;
    let schools = comcigan::blocking::search_school(&"향동중", data.clone())?;
    let school = comcigan::blocking::view(&schools[0], data.clone())?;
    let study = school.grade(2).class(4).day(5).study(4);
    println!("Subject: {}", study.subject);
    println!("Teacher: {}", study.teacher);
    let then = Instant::now();

    println!("Time elapsed: {}", then.duration_since(now).as_millis());
    Ok(())
}

#[tokio::test]
#[cfg_attr(not(feature = "blocking"), ignore)]
async fn test_promise() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::time::Instant;
    use hyper::Client;


    let now = Instant::now();
    let client = Client::new();
    let data = comcigan::promise::request_target(&client).await?;
    let schools = comcigan::promise::search_school(&client, "향동중", data.clone()).await?;
    let school = comcigan::promise::view(&client, &schools[0], data.clone()).await?;
    let study = school.grade(2).class(4).day(5).study(4);
    println!("Subject: {}", study.subject);
    println!("Teacher: {}", study.teacher);
    let then = Instant::now();

    println!("Time elapsed: {}", then.duration_since(now).as_millis());
    Ok(())
}
