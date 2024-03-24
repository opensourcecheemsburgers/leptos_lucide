use leptos::*;

use crate::LucideAttributesCtx;

#[component]
pub fn J.rs() -> impl IntoView {
	let attributes = move || expect_context::<LucideAttributesCtx>().0.get();

    view! {
        <svg
            class=attributes().classes()
            xmlns=attributes().xmlns()
            width=attributes().width()
            height=attributes().height()
            viewBox=attributes().view_box()
            fill=attributes().fill()
            stroke=attributes().stroke()
            stroke-width=attributes().stroke_width()
            stroke-linecap=attributes().stroke_linecap()
            stroke-linejoin=attributes().stroke_linejoin()
        ></svg>
    }
}