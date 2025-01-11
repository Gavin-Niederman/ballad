pub mod bus;
mod gamma;

fn main() {
    smol::spawn(gamma::ClientState::run()).detach();
    smol::block_on(async {
        let _connection = zbus::connection::Builder::session()
            .expect("Failed to create session connection")
            .name("com.gavinniederman.BalladDisplayCfg")
            .expect("Failed to reserve interface name")
            .serve_at(
                "/com/gavinniederman/BalladDisplayCfg",
                bus::BalladDisplayCfg,
            )
            .expect("Failed to serve interface")
            .build()
            .await
            .unwrap();
        smol::future::pending::<()>().await;
    });
}
