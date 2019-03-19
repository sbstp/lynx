use lynx::HttpResult;

fn main() -> HttpResult {
    env_logger::init();

    let (status, headers, reader) = lynx::post("https://httpbin.org/post").body("hello, world!").send()?;
    println!("Status: {:?}", status);
    println!("Headers:\n{:#?}", headers);
    println!("Body:\n{}", reader.string()?);

    Ok(())
}
