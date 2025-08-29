use tokio::{
    task::JoinError,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    let executor = tokio::spawn(async move {
        for i in 1..10 {
            println!("Hi n*{} from the new task!", i);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    if let (Err(e),) = tokio::join!(executor) {
        eprintln!("Error {e}");
    }

    let main_task = for i in 1..5 {
        println!("Hi n*{i} from the main task!");
    };

    println!("Main task is done! {main_task:?}")
}
