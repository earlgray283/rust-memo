use std::time::Duration;
use async_std::task;

#[async_std::main]
async fn main() {
    // await をしなくても既に実行されている
    let handle = task::spawn(
        async {
        task::sleep(Duration::from_secs(2)).await;
        println!("from async block");
    });

    // handle と同時に実行される。
    std::thread::sleep(Duration::from_secs(2));

    // handle が終わるまで待つ。
    // この場合、2s後には handle が終わっているはずだから await しなくてもいい
    let _ = handle.await;
    
    // from async block と from main が同時に表示される
    println!("from main");
}
