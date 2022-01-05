## Let's build 'dodge' with Bery-0.6 (in Japanese)

- スプライトアニメーションは、SpriteAtlasに一つずつロード。定期的に切り替えていけばアニメーションにはなる。ease in/outがしたいなら自分でなんとかするしかない。
- 自機の移動は、？
- 敵の発生は別にgeneratorはいらないだろう。乱数で適当に振って、画面の外に追い出して、中を向かせてやればいい。
- 描画領域外に出たときに`despawn`するのは無駄だろう。enemy poolを作って再初期化する方がいいのでそうしました。
- BGM：同梱のプラグインはmp3だけでなくoggも再生できるけどリピート再生できないので、[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)に変えるしかなさそう。あるいは曲長のタイマーを仕掛けるしか。。。


