# Revising Snake Sample for Bevy 0.X (in Japanese)

- https://mbuffett.com/posts/bevy-snake-tutorial/

最新のBevy(main branch)をキャッチアップ。

## **An empty bevy app** and **Createing a window**

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


