use tokio::time::{self, Duration, Instant};

async fn requete_api(url: &str, delay_ms: u64) -> String {
    time::sleep(Duration::from_millis(delay_ms)).await;
    format!("Donnee recue de {}", url)
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let api_a = requete_api("API A", 100);
    let api_b = requete_api("API B", 300);
    let api_c = requete_api("API C", 400);

    let (res_a, res_b, res_c) = tokio::join!(api_a, api_b, api_c);

    println!("A= {}, B={}, C={}", res_a, res_b, res_c);

    let duration = start.elapsed();
    println!(
        "Toutes les requetes sont terminees avec un delai de {:?} seconds",
        duration.as_secs_f32()
    )
}
