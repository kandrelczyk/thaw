# Scrollbar

```rust demo
view! {
    <Scrollbar style="max-height: 80px;">
        <p style="line-height: 1.5">
            r#"This being said, the world is moving in the direction opposite to Clarke's predictions. In 2001: A Space Odyssey, in the year of 2001, which has already passed, human beings have built magnificent cities in space, and established permanent colonies on the moon, and huge nuclear-powered spacecraft have sailed to Saturn. However, today, in 2018, the walk on the moon has become a distant memory.And the farthest reach of our manned space flights is just as long as the two-hour mileage of a high-speed train passing through my city. At the same time, information technology is developing at an unimaginable speed. With the entire world covered by the Internet, people have gradually lost their interest in space, as they find themselves increasingly comfortable in the space created by IT. Instead of an exploration of the real space, which is full of real difficulties, people now just prefer to experience virtual space through VR. Just like someone said, "You promised me an ocean of stars, but you actually gave me Facebook.""#
        </p>
    </Scrollbar>
}
```

### Scroll horizontally

```rust demo
view! {
    <Scrollbar>
        <p style="line-height: 1.5; white-space: nowrap; padding: 12px;">
            r#"This being said, the world is moving in the direction opposite to Clarke's predictions. In 2001: A Space Odyssey, in the year of 2001, which has already passed, human beings have built magnificent cities in space, and established permanent colonies on the moon, and huge nuclear-powered spacecraft have sailed to Saturn. However, today, in 2018, the walk on the moon has become a distant memory.And the farthest reach of our manned space flights is just as long as the two-hour mileage of a high-speed train passing through my city. At the same time, information technology is developing at an unimaginable speed. With the entire world covered by the Internet, people have gradually lost their interest in space, as they find themselves increasingly comfortable in the space created by IT. Instead of an exploration of the real space, which is full of real difficulties, people now just prefer to experience virtual space through VR. Just like someone said, "You promised me an ocean of stars, but you actually gave me Facebook.""#
        </p>
    </Scrollbar>
}
```

### Scrollbar Props

| Name          | Type                                     | Default              | Description                |
| ------------- | ---------------------------------------- | -------------------- | -------------------------- |
| class         | `MaybeProp<String>`                      | `Default::default()` |                            |
| style         | `MaybeProp<String>`                      | `Default::default()` |                            |
| content_class | `MaybeProp<String>`                      | `Default::default()` | Class name of content div. |
| content_style | `MaybeProp<String>`                      | `Default::default()` | Style of content div.      |
| size          | `u8`                                     | `8`                  | Size of scrollbar.         |
| comp_ref      | ref `Option<ComponentRef<ScrollbarRef>>` | `None`               |                            |
| children      | `Children`                               |                      |                            |

### ScrollbarRef Props

| Name                             | Type                                            | Description     |
| -------------------------------- | ----------------------------------------------- | --------------- |
| scroll_to_with_scroll_to_options | `Fn(&self, options: &web_sys::ScrollToOptions)` | Scroll content. |
