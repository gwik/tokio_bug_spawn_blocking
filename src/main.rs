use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(2048)
        .thread_name("tokio-runtime")
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    runtime.block_on(async {
        let mut handles = Vec::new();

        for i in 0..1024 {
            eprintln!("Try spawning {}", i);
            let handle = tokio::task::spawn_blocking(move || {
                let thread = std::thread::current();
                eprintln!(
                    "Running {} on {:?}/{}",
                    i,
                    thread.id(),
                    thread.name().unwrap_or_default(),
                );
                std::thread::sleep(std::time::Duration::from_secs(1));
                eprintln!("Ran {}", i);
            });
            handles.push(handle);
        }

        eprintln!("waiting for spawn tasks to complete...");

        for handle in handles {
            handle.await.expect("success");
        }
    });
}
