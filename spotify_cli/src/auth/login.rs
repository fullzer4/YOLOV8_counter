use reqwest::{Client, Error, Response};
use webbrowser;
use rouille;

pub fn Login() {
    let client_id = "42c67d6d685a4a9095075a33a8c94eb3";
    let redirect_uri = "http://localhost:8080/callback";

    let authorize_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private",
        client_id, redirect_uri
    );

    if webbrowser::open(&authorize_url).is_ok() {
        let client_secret = "seu_client_secret";

        let client = Client::new();
        let token_url = "https://accounts.spotify.com/api/token";
        
        rouille::start_server("localhost:8080", move |request| {
            let mut callback_url = request.url().to_owned();
            let code = callback_url
                .split('?')
                .nth(1)
                .and_then(|query| {
                    let params: Vec<_> = query.split('&').collect();
                    params
                        .into_iter()
                        .find(|param| param.starts_with("code="))
                        .map(|param| param.trim_start_matches("code="))
                });

            if let Some(code) = code {
                let body = format!(
                    "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
                    code, redirect_uri, client_id, client_secret
                );

                async {
                    let response = fetch_token(&client, token_url, body).await.expect("Erro na requisição");
                    let token: TokenResponse = response.json().expect("Erro ao ler a resposta JSON");
                    println!("Token de acesso: {}", token.access_token);
                };

                rouille::Response::text("Login concluído! Feche esta janela e volte para o terminal.").with_status_code(200)
            } else {
                rouille::Response::text("Código de autorização não encontrado na URL.").with_status_code(400)
            }
        });
    }
}

async fn fetch_token(client: &Client, token_url: &str, body: String) -> Result<Response, Error> {
    client
        .post(token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
}
