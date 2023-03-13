use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    #[cfg(not(feature="hydrate"))]
    {
        log::debug!("render application component");
        sycamore::render(App);
    }

    #[cfg(feature="hydrate")]
    {
        log::debug!("hydrate application component");
        sycamore::hydrate(App);
    }
}
