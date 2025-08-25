use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    tokio::select! {
        europe = load_from_europe() => {
            println!("{}", europe);
        },

        america = load_from_america() => {
            println!("{}", america);
        },


    }
}

async fn load_from_europe() -> String {
    sleep(Duration::from_secs(4)).await;
    "European server reached. Transfert engaged.".to_string()
}

async fn load_from_america() -> String {
    sleep(Duration::from_secs(4)).await;
    "American server reached. Transfert engaged.".to_string()
}
