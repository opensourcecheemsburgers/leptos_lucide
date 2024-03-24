# Lucide Icons in Leptos

This library provides Leptos components for Lucide SVGs .

## Installation

From the project directory, use the following command:

```
cargo add leptos_lucide
``` 


Alternatively, add the following line to your `Cargo.toml`:

```
leptos_lucide = "0.1.1"
```


## Usage

Create a `LucideAttributes` struct in a parent component. Then, use `provide_context()` to pass the attributes to any Lucide icon.

### Method 1: `new_with_attributes()`

```
use leptos::*;
use leptos_lucide::icons::*;

#[component]
pub fn SomeComponent() -> impl IntoView {
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
    let attributes_ctx = LucideAttributesCtx(RwSignal::new(attributes));
    provide_context(attributes_ctx);

    view! {
        <Wallet/>
        <WalletCards/>
        <WalletMinimal/>
    }
}
```

### Method 2: `new()` - Builder Pattern

```
use leptos::*;
use leptos_lucide::icons::*;

#[component]
pub fn ExampleComponent() -> impl IntoView {
    let attributes = LucideAttributes::new()
        .set_height(24)
        .set_width(96)
        .set_stroke_width(1.625);
    let attributes_ctx = LucideAttributesCtx(RwSignal::new(attributes));
    provide_context(attributes_ctx);
    
    view! {
        <Wallet/>
        <WalletCards/>
        <WalletMinimal/>
    }
}
```
