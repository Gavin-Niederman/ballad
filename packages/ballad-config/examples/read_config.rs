use ballad_config::ShellConfig;

fn main() {
    let config = ballad_config::get_or_init_shell_config();
    println!("Config: {:?}", config);
    ballad_config::set_shell_config(&ShellConfig {
        catppuccin_flavor: ballad_config::CatppuccinFlavor::Mocha,
    });
}