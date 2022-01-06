## Let's build 'dodge' with Bery-0.6 (in Japanese)

### Sprite animation

スプライトアニメーションは、SpriteAtlasに一つずつロード。定期的に切り替えていけばアニメーションにはなる。
- (2022-01-05) ただし、ロードが終了するまでゲーム開始を待ってくれないので、準備が終わったかどうかをstageを使って表現しないといけない。

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

- (2022-01-06) Resourceとして最初にロードしてしまえば、stagingは要らないんじゃないかと思ったのだが、勘違いだったようだ。Bevyのmanualから引用するけど、使う前にロードできるとは書いてない(`AssetServer`がいないのだ。`DefaultPlugins`の後ならありそうな気がしたのだけど甘かった)。

> A resource in Bevy represents globally unique data. Resources must be added to Bevy Apps before using them. This happens with insert_resource. (pub fn insert_resource)

- (2022-01-07)

[examples/shader/shader_instancing.rs](https://github.com/bevyengine/bevy/blob/507441d96f83355cdab578d85f804f4bf8d835c9/examples/shader/shader_instancing.rs#L154-L168)からこんなのを見つけたのだが、やっぱりリソースが読み込まれるまでstageで待たないといけなかった。

```
impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        asset_server.watch_for_changes().unwrap();
        let shader = asset_server.load("shaders/instancing.wgsl");


        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();


        CustomPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}
```


### F-curve (ease in/out)

アニメーションのease in/outがしたいなら自分でなんとかするしかない。

### Input handling

自機の移動。入力と内部世界、内部世界と出力に切り分けてみた。

- 入力系

マウスポジションを内部座標系に変換するのはちょっとtrickyなことをしなければいけないらしい。
なので、入力ハンドラがカメラを要求するというちょっと、な感じになった。

```rust
fn track_mouse_movement(
    windows: ResMut<Windows>,
    mut queries: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Character, With<Player>>,
    )>,
)
```

- 表示系

表示系はリアルタイムでキャラクタの座標を書き換えていくので反映させるために`&mut Character`が必要。
また、ゆっくりとスプライトを入れ替えていくのに`Timer`を使ったのだが、`FixedTimestep`とどちらがいいのだろう。

```rust
fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Character,
            &mut Timer,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
)
```

### Spawn enemies

敵の発生は別にgeneratorはいらないだろう。乱数で適当に振って、画面の外に追い出して、中を向かせてやればいい。

描画領域外に出たときに`despawn`するのは無駄だろう。enemy poolを作って再初期化する方がいいのでそうしました。そもそも`despawn`はstageを切り替えるタイミングで実行されるということなので、ゲーム前、ゲーム中といった荒いステージ分けでは意味がない。

### BGM

BGM：同梱のプラグインはmp3だけでなくoggも再生できるけどリピート再生できないので、[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)に変えるしかなさそう。あるいは曲長のタイマーを仕掛けるしか。。。
