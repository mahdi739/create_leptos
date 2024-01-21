pub mod app;
// #[cfg(all(feature = "ssr", feature = "hydrate"))]
// compile_error!("feature \"ssr\" and feature \"hydrate\" cannot be enabled at the same time");

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
  println!("horray");
  console_error_panic_hook::set_once();
  leptos::mount_to_body(app::App);
}

