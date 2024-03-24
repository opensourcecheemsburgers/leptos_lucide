use std::{borrow::Cow, rc::Rc};

use leptos::{provide_context, RwSignal};

pub mod icons;

#[cfg(test)]
mod tests;

static DEFAULT_XMLNS: &'static str = "http://www.w3.org/2000/svg";
static DEFAULT_WIDTH: &'static str = "16";
static DEFAULT_HEIGHT: &'static str = "16";
static DEFAULT_VIEW_BOX: &'static str = "0 0 24 24";
static DEFAULT_FILL: &'static str = "none";
static DEFAULT_STROKE: &'static str = "currentColor";
static DEFAULT_STROKE_WIDTH: &'static str = "2";
static DEFAULT_STROKE_LINECAP: &'static str = "round";
static DEFAULT_STROKE_LINEJOIN: &'static str = "round";

#[derive(Clone, Copy)]
pub struct LucideAttributesCtx(pub RwSignal<LucideAttributes>);

/// # LucideAttributes
///
/// An attributes struct for `lucide-leptos` SVG components. It is internally reference-counted, so
/// it is cheap to clone.
///
/// ## Instantiate
///
/// There are two options availables to create a `LucideAttributes`.
///
/// Use `new_with_attributes()` to create a full attribute list from scratch:
///
/// Alternatively, use the builder method `new()`, if you only wish to set a few attributes.
///
/// ## Tips
///
/// For applications with a universal icon look, provide a `LucideAttributes` context via
/// `provide_context::<LucideAttributes>()`.

#[derive(Clone)]
pub struct LucideAttributes {
	classes: Rc<str>,
	xmlns: Rc<str>,
	width: Rc<str>,
	height: Rc<str>,
	view_box: Rc<str>,
	fill: Rc<str>,
	stroke: Rc<str>,
	stroke_width: Rc<str>,
	stroke_linecap: Rc<str>,
	stroke_linejoin: Rc<str>,
}

impl Default for LucideAttributes {
	fn default() -> Self {
		Self {
			classes: Rc::from(""),
			xmlns: Rc::from(DEFAULT_XMLNS),
			width: Rc::from(DEFAULT_WIDTH),
			height: Rc::from(DEFAULT_HEIGHT),
			view_box: Rc::from(DEFAULT_VIEW_BOX),
			fill: Rc::from(DEFAULT_FILL),
			stroke: Rc::from(DEFAULT_STROKE),
			stroke_width: Rc::from(DEFAULT_STROKE_WIDTH),
			stroke_linecap: Rc::from(DEFAULT_STROKE_LINECAP),
			stroke_linejoin: Rc::from(DEFAULT_STROKE_LINEJOIN),
		}
	}
}

impl LucideAttributes {
	/// Creates a new `LucideAttributes` with the given arguments.
	///
	/// For string-like attributes, you can pass anything that is `ToString`.
	///
	/// ```
	/// LucideAttributes::new_with_attributes(
	/// 	"animate-pulse",
	/// 	"http://www.w3.org/2000/svg",
	/// 	24,
	/// 	24,
	/// 	"0 0 24 24",
	/// 	"#ffffff",
	/// 	"rgba(0,0,0,0.69)",
	/// 	1.625,
	/// 	"round",
	/// 	"round",
	/// );
	/// ```
	///
	/// **Note**: If you only wish to set a few attributes, use `new()` instead.
	pub fn new_with_attributes(
		classes: &str,
		xmlns: &str,
		width: &str,
		height: &str,
		view_box: &str,
		fill: &str,
		stroke: &str,
		stroke_width: &str,
		stroke_linecap: &str,
		stroke_linejoin: &str,
	) -> Self {
		Self {
			classes: Rc::from(classes),
			xmlns: Rc::from(xmlns),
			width: Rc::from(width),
			height: Rc::from(height),
			view_box: Rc::from(view_box),
			fill: Rc::from(fill),
			stroke: Rc::from(stroke),
			stroke_width: Rc::from(stroke_width),
			stroke_linecap: Rc::from(stroke_linecap),
			stroke_linejoin: Rc::from(stroke_linejoin),
		}
	}

	/// Creates a new `LucideAttributes`` with the default Lucide SVG attributes.
	///
	/// Edit individual attributes using their respective `set()` functions.
	///
	/// ```
	/// LucideAttributes::new().set_height(24).set_width(96).set_stroke_width(1.625)
	/// ```
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns a reference-counted pointer to the `xmlns` attribute.
	pub fn xmlns(&self) -> Rc<str> {
		self.xmlns.clone()
	}

	/// Set the SVG's `xmlns` attribute. You can pass anything that is `ToString`.
	pub fn set_xmlns(&mut self, xmlns: &str) -> Self {
		self.xmlns = Rc::from(xmlns);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `classes` attribute.
	pub fn classes(&self) -> Rc<str> {
		self.classes.clone()
	}

	/// Set the SVG's `class` attribute. You can pass anything that is `ToString`.
	pub fn set_classes(&mut self, classes: &str) -> Self {
		self.classes = Rc::from(classes);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `width` attribute.
	pub fn width(&self) -> Rc<str> {
		self.width.clone()
	}

	/// Set the SVG's `width` attribute in pixels. You can pass anything that is `ToString`.
	pub fn set_width(&mut self, width: &str) -> Self {
		self.width = Rc::from(width);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `height` attribute.
	pub fn height(&self) -> Rc<str> {
		self.height.clone()
	}

	/// Set the SVG's `height` attribute in pixels. You can pass anything that is `ToString`.
	pub fn set_height(&mut self, height: &str) -> Self {
		self.height = Rc::from(height);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `view_box` attribute.
	pub fn view_box(&self) -> Rc<str> {
		self.view_box.clone()
	}

	/// Set the SVG's `view_box` attribute. You can pass anything that is `ToString`.
	pub fn set_view_box(&mut self, view_box: &str) -> Self {
		self.view_box = Rc::from(view_box);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `fill` attribute.
	pub fn fill(&self) -> Rc<str> {
		self.fill.clone()
	}

	/// Set the SVG's `fill` attribute. You can pass anything that is `ToString`.
	pub fn set_fill(&mut self, fill: &str) -> Self {
		self.fill = Rc::from(fill);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `stroke` attribute.
	pub fn stroke(&self) -> Rc<str> {
		self.stroke.clone()
	}

	/// Set the SVG's `stroke` attribute. You can pass anything that is `ToString`.
	pub fn set_stroke(&mut self, stroke: &str) -> Self {
		self.stroke = Rc::from(stroke);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `stroke_width` attribute.
	pub fn stroke_width(&self) -> Rc<str> {
		self.stroke_width.clone()
	}

	/// Set the SVG's `stroke_width` attribute using an `f64`.
	pub fn set_stroke_width(&mut self, stroke_width: &str) -> Self {
		self.stroke_width = Rc::from(stroke_width);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `stroke_linecap` attribute.
	pub fn stroke_linecap(&self) -> Rc<str> {
		self.stroke_linecap.clone()
	}

	/// Set the SVG's `stroke_linecap` attribute. You can pass anything that is `ToString`.
	pub fn set_stroke_linecap(&mut self, stroke_linecap: &str) -> Self {
		self.stroke_linecap = Rc::from(stroke_linecap);
		self.clone()
	}

	/// Returns a reference-counted pointer to the `stroke_linejoin` attribute.
	pub fn stroke_linejoin(&self) -> Rc<str> {
		self.stroke_linejoin.clone()
	}

	/// Set the SVG's `stroke_linejoin` attribute. You can pass anything that is `ToString`.
	pub fn set_stroke_linejoin(&mut self, stroke_linejoin: &str) -> Self {
		self.stroke_linejoin = Rc::from(stroke_linejoin);
		self.clone()
	}
}
