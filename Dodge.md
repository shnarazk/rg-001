## Let's build 'dodge' with Bery-0.6 (in Japanese)

- スプライトアニメーションは、SpriteAtlasに一つずつロード。定期的に切り替えていけばアニメーションにはなる。ただし、ロードが終了するまでゲーム開始を待ってくれないので、準備が終わったかどうかをstageを使って表現しないといけない。

```rust
    .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
    .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
    .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_cammera))
    ...

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<CharacterSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Ready).unwrap();
    }
}
```

- アニメーションのease in/outがしたいなら自分でなんとかするしかない。
- 自機の移動は、？
- 敵の発生は別にgeneratorはいらないだろう。乱数で適当に振って、画面の外に追い出して、中を向かせてやればいい。
- 描画領域外に出たときに`despawn`するのは無駄だろう。enemy poolを作って再初期化する方がいいのでそうしました。そもそも`despawn`はstageを切り替えるタイミングで実行されるということなので、ゲーム前、ゲーム中といった荒いステージ分けでは意味がない。
- BGM：同梱のプラグインはmp3だけでなくoggも再生できるけどリピート再生できないので、[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)に変えるしかなさそう。あるいは曲長のタイマーを仕掛けるしか。。。


