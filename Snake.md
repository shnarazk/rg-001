# Revising Snake Sample for Bevy 0.X (in Japanese)

- https://mbuffett.com/posts/bevy-snake-tutorial/

最新のBevy(main branch)をキャッチアップ。

## '**An empty bevy app**' and '**Createing a window**'

- `App`は`build()`ではなく`new()`で生成する。
- `add_*_system`において`.system()`は不要になった。

```diff
  fn main() {
-     App::build()
+     App::new()
           ...
          .add_plugins(DefaultPlugins)
-         .add_startup_system(setup.system())
+         .add_startup_system(setup)
          .add_startup_system(spawn_snake)
-         .add_system(position_translation.system())
+         .add_system(position_translation)
```

https://github.com/shnarazk/rg-001/blob/a0b55ab9cf1b261e02694cfbb20ae1f1393e2efe/src/bin/snake.rs#L10-L20

## '**The beginnings of a snake**'

- `ColorMaterial`はなくなったので`Color`で代用する。
- `SpriteBundle`はtextureベースになり、`material`や`sprite`がなくなった。`material`は`color`で、`sprite`は`custom_size`で置き換える。`custom_size`は`Option`でwrapされていることに注意。

```diff
  fn spawn_segment(mut commands: Commands, materials: Res<Materials>, position: Position) -> Entity {
      commands
          .spawn_bundle(SpriteBundle {
              sprite: Sprite {
-                 material: materials.head_material.clone(),
+                 color: materials.segment_material,
-                 sprite: Sprite::new(Vec2::new(10.0, 10.0)),
+                 custom_size: Some(Vec2::new(10.0, 10.0)),
                  ..Default::default()
              },
              ..Default::default()
          })
          .insert(SnakeSegment)
          .insert(position)
          .insert(Size::square(0.6))
          .id()
}
```

