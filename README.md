# AviUtl-AdjustLastFlame-rs

習作

## 概要
これは[AviUtl プラグイン - 最終フレーム自動調整 version 1.0.3](https://github.com/hebiiro/AviUtl-Plugin-AdjustLastFrame) ([by 蛇色](https://github.com/hebiiro))を出来る限りRustで再現するために書いたAviUtlプラグインです。挙動、導入、使い方については上述のリンクにてご確認ください。

## 注意
このプログラムの多く(ほぼ全体)が`unsafe`であり、また作成者自身メモリ操作を熟知していないため、このプラグインを使用することによって予期せぬクラッシュが発生する場合があります。~~また、原作の方がバイナリサイズが圧倒的に小さいため、原作の方を使用することをおすすめします。~~ 現在最小で12.5KBです。
