use anyhow::{Ok, Result};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Response};

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();

    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }

        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    let res = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse::<Mime>().unwrap());

    res
}

pub async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    print_body(get_content_type(&resp), &resp.text().await?);

    Ok(())
}
