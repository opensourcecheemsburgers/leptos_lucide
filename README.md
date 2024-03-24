# Lucide Icons in Leptos

A crate that generates Leptos components of Lucide SVGs.

## Installation

From the project directory, use the following command:

```
cargo add lucide-leptos
``` 


Alternatively, add the following line to your `Cargo.toml`:

```
lucide-leptos = "0.1.0"
```

## Usage

To use the Lucide SVG components from `leptos_lucide` you will need to provide `LucideAttributes` struct to your component.

### Method 1: `new_with_attributes()`

Create a `LucideAttributes` struct like so:

```
use leptos::*;
use lucide-leptos::icons::Server;

#[component]
pub fn ExampleComponent() -> impl IntoView {
    let attributes = LucideAttributes::new_with_attributes(
        "animate-pulse", 
        "http://www.w3.org/2000/svg",
        24, 
        24, 
        "0 0 24 24", 
        "#ffffff", 
        "rgba(0,0,0,0.69)", 
        1.625,
        "round",
        "round"
    );

    view! {
        <div>
            <Server attributes=attributes>
        </div>
    }
}
```

You can also create a static `LucideAttributes`.

```
use leptos::*;
use lucide-leptos::icons::Server;

static ATTRIBUTES: LucideAttributes = LucideAttributes::new_with_attributes(
    "animate-pulse", 
    "http://www.w3.org/2000/svg",
    24, 
    24, 
    "0 0 24 24", 
    "#ffffff", 
    "rgba(0,0,0,0.69)", 
    1.625,
    "round",
    "round"
);

#[component]
pub fn ExampleComponent() -> impl IntoView {
    view! {
        <div>
            <Server attributes=ATTRIBUTES>
        </div>
    }
}
```

### Method 2: `new()` - Builder Pattern

You can also create a `LucideAttributes` using `LucideAttributes::new()`.

Then, you can set individual attributes using their respective `set()` functions.

```
use leptos::*;
use lucide-leptos::icons::Server;

#[component]
pub fn ExampleComponent() -> impl IntoView {
    let attributes = LucideAttributes::new()
        .set_height(24)
        .set_width(96)
        .set_stroke_width(1.625);
    
    view! {
        <div>
            <Server attributes=attributes>
        </div>
    }
}
```

This is useful if you only wish to set a few parameters. 

**Note:** Any unset values will be set to their respective Lucide defaults.

### Tips

`LucideAttributes` is internally reference-counted so it is cheap to clone.

If you use similar SVG attributes throughout your app, provide `LucideAttributes` as a context.

```
use leptos::*;
use lucide-leptos::icons::Server;

static APP_ICON_ATTRIBUTES: LucideAttributes = LucideAttributes::new_with_attributes(
    "animate-pulse", 
    "http://www.w3.org/2000/svg",
    24, 
    24, 
    "0 0 24 24", 
    "#ffffff", 
    "rgba(0,0,0,0.69)", 
    1.625,
    "round",
    "round"
);

#[component]
pub fn ParentComponent() -> impl IntoView {
    provide_context::<LucideAttributes>(APP_ICON_ATTRIBUTES);

    view! {
        // lots of children with Lucide icons
    }
}
```
