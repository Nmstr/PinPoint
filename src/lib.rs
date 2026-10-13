#![allow(non_snake_case)]

mod app;

pub use app::App;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _guard = runtime.enter();

    App::new().unwrap().run().unwrap();
}
