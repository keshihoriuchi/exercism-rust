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

* shiro (参考): `127.0.0.1` をバインドするように変えて `/` をGET
* hayashi (参考): `127.0.0.1` をバインドするように変えて `/counter` をGET

数字が入っているセルは
* `/` の左: `Time per request [ms] (mean)`
* `/` の右: `Time per request [ms] (mean, across all concurrent requests)`

|       | -c 1 | -c 16 | -c 160 | -c 1 -k | -c 16 -k | -c 160 -k |
|:------|---:|------:|-------:|--------:|---------:|----------:|
| shiro (参考) | 0.133 / 0.133 | 0.876 / 0.55 | 負荷に耐えられない | 非対応 | 非対応 | 非対応 |
| seikichi | 0.127 / 0.127 | 0.783 / 0.049 | 8.166 / 0.51 | 0.126 / 0.126 | 0.818 / 0.051 | 8.924 / 0.056 |
| fuka | 0.124 / 0.124 | 0.726 / 0.045 | 19.108 / 0.119 | 0.043 / 0.043 | 0.200 / 0.013 | 2.157 / 0.013 |
| horiuchi | 0.129 / 0.129 | 1.034 / 0.065 | 負荷に耐えられない | 非対応 | 非対応 | 非対応 |
| hayashi (参考) | 0.220 / 0.220 | 1.810 / 0.113 | 負荷に耐えられない | 非対応 | 非対応 | 非対応 |
