# WAF課題ベンチマーク

## テスト手順

### 概要

[TechEmpowerのJSON Serialization](https://github.com/TechEmpower/FrameworkBenchmarks/wiki/Project-Information-Framework-Tests-Overview#json-serialization) に沿ってテストする。

Apache Benchリクエストする。クライアントとサーバは同一マシンで測定する。 

### バリエーション

* リクエスト数: 2400
* クライアント数: 1, 16, 160
* Keep-Alive: あり, なし

### 測定環境

* Apach Bench 2.3
* rust toolchain: nightly-2019-10-04-x86_64-apple-darwin
* OS: Mac 10.14.6
* CPU: Core i5 2.3GHz
* メモリ: 16GB 2133 MHz LPDDR3

## 結果

### 概要

|       | -c 1 | -c 16 | -c 160 | -c 1 -k | -c 16 -k | -c 160 -k | 備考 |
|:------|---:|------:|-------:|--------:|---------:|----------:|:-----|
| shiro (参考) | - | - | - | - | - | - |  |
| seikichi | - | - | - | - | - | - |  |
| fuka | - | - | - | - | - | - |  |
| horiuchi | - | - | - | - | - | - |  |
| hayashi | - | - | - | - | - | - |  |