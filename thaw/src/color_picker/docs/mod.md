# Color Picker

```rust demo
use palette::Srgb;

let value = RwSignal::new(Color::from(Srgb::new(0.0, 0.0, 0.0)));

view! {
    <ColorPicker value/>
}
```

### Color Format

Encoding formats, support RGB, HSV, HSL.

```rust demo
use palette::{Hsl, Hsv, Srgb};

let rgb = RwSignal::new(Color::from(Srgb::new(0.0, 0.0, 0.0)));
let hsv = RwSignal::new(Color::from(Hsv::new(0.0, 0.0, 0.0)));
let hsl = RwSignal::new(Color::from(Hsl::new(0.0, 0.0, 0.0)));

view! {
    <Space vertical=true>
        <ColorPicker value=rgb/>
        <ColorPicker value=hsv/>
        <ColorPicker value=hsl/>
    </Space>
}
```

### ColorPicker Props

| Name  | Type                | Default              | Desciption           |
| ----- | ------------------- | -------------------- | -------------------- |
| class | `MaybeProp<String>` | `Default::default()` |                      |
| value | `Model<Color>`      | `Default::default()` | Value of the picker. |
