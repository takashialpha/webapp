// main.rs
mod server;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(server::run());
}

