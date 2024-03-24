use leptos::view;
use wasm_bindgen_test::*;

use crate::icons::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn mount_svgs() {
	leptos::mount_to_body(move || {
		view! { <Youtube/> }
	});
}
