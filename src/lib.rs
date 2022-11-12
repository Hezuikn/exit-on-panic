pub fn once_exit_on_panic() {
    static ONCE_GUARD: std::sync::Once = std::sync::Once::new();
    ONCE_GUARD.call_once(|| {
        let org = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            org(info);
            std::process::exit(101);
        }));
    });
}
