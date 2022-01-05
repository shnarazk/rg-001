# A study of graphical and interactive application development in Rust

あるいは来るべき[Bevy 0.6](https://bevyengine.org)（Rust製ゲームエンジン）に備えるリポジトリ。

0.5がリリースされてからはや9ヶ月。
これまでのリリースペースに比べるとまるで開発が止まってしまったかのように見えますが、[リポジトリ](https://github.com/bevyengine/bevy)やdiscordを覗くと0.6のリリースが遠い未来の話ではないことがわかります。
採用を検討する価値あり。

- https://bevyengine.org
- https://github.com/bevyengine/bevy
- https://bevy-cheatbook.github.io/introduction.html

## Adapt 'Dodge' (your first Godot game) to the latest Bevy

ほぼ移植終了。機能追加検討中。

```
cargo run --release --bin dodge
```

Wait. Before running it, you need to put the assets from Godot-doc site.

1. get [dodge_assets.zip](https://docs.godotengine.org/en/stable/_downloads/e79a087a28c8eb4d140359198a122c0f/dodge_assets.zip)
1. put the contents under `assets/dodge/`

- [メモ;Notes in Japanese](https://github.com/shnarazk/rg-001/blob/main/Dodge.md) (under construction)
- [Your first game](https://docs.godotengine.org/en/stable/getting_started/step_by_step/your_first_game.html) with [Godot](https://godotengine.org/)

## Adapt Snake to the latest Bevy

移植終了。

```
cargo run --release --bin snake
```

- [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/)
- [変更点;Updating Snake in Japanese](https://github.com/shnarazk/rg-001/blob/main/Snake.md)

---

### other links

- https://github.com/rg3dengine/rg3d
- https://docs.godotengine.org/en/stable/getting_started/step_by_step/your_first_game.html
