fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let initial_post_url = inquire::Text::new("A:").prompt()?;

    let initial_post_url = "https://www.youtube.com/post/Ugkx4cdNJgZPYbhSLRehjUHywEwBNlpn7A_f?lc=UgyEejCqC_ilo-zLj_N4AaABAg&si=T7DNr0-442WYtlKS" ;

    let mut post_url = url::Url::parse(&initial_post_url)?;
    post_url.set_query(None);

    let resp_text = reqwest::blocking::get(post_url)?.text()?;

    let needle = r#"<meta property="og:image" content=""#;
    let start = resp_text.find(needle).ok_or("og:image not found")? + needle.len();
    let rest = &resp_text[start..];
    let end = rest.find("=s").ok_or("FOUND INVALID OG IMAGE ADDRESS")?;

    let final_img_url = format!("{}0", &rest[..(end + 2)]);

    let img_resp = reqwest::blocking::get(&final_img_url)?;

    println!("{:?}", img_resp.headers().get(reqwest::header::CONTENT_TYPE));

    Ok(())
}

// pub fn sanitize
