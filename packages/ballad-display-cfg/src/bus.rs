use zbus::interface;

pub struct BalladDisplayCfg;
#[interface(
    name = "com.gavinniederman.BalladDisplayCfg",
    proxy(
        default_service = "com.gavinniederman.BalladDisplayCfg",
        default_path = "/com/gavinniederman/BalladDisplayCfg",
    )
)]
impl BalladDisplayCfg {
    async fn set_brightness(&self, _brightness: f64) {
        todo!()
    }
}
