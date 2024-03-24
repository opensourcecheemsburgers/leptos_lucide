use leptos::*;
use wasm_bindgen_test::*;

use crate::icons::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn mount_svgs() {
	mount_to_body(move || {
		view! { <Youtube/> }
	});
}
