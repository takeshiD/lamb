# 🐑lamb - Scheme implementation by Rust
  - [文法](#文法)
    - [データ型](#データ型)
    - [基本的な演算子](#基本的な演算子)
    - [変数・関数定義](#変数関数定義)
    - [制御構造](#制御構造)
    - [リスト操作](#リスト操作)
  - [実装予定の機能](#実装予定の機能)

lambはRustで実装されたシンプルなScheme処理系です。[R5RS準拠](https://www.unixuser.org/~euske/doc/r5rs-ja/r5rs-ja.pdf)のサブセットを実装しています。
このREADMEでは

1. 基本的なデータ型の説明
2. 主要な演算子と構文の説明
3. 変数・関数定義の方法
4. リスト操作の基本
5. 実装予定の機能リスト

を記載しています。

## 文法
### データ型
- 整数: `42`, `-7`
- 浮動小数点数: `3.14`, `-0.5`
- シンボル: `x`, `add!`, `string->number`
- 真偽値: `#t`, `#f`
- リスト: `(1 2 3)`, `(+ 1 2)`
- 文字列: `"hello"`, `"scheme"`
- 文字: `#\a`, `#\space`

### 基本的な演算子
- 数値演算: `+`, `-`, `*`, `/`, `quotient`, `remainder`
- 比較演算: `=`, `<`, `>`, `<=`, `>=`
- 論理演算: `and`, `or`, `not`

```scheme
(+ 1 2)        ; => 3
(* 4 5)        ; => 20
(quotient 10 3) ; => 3
(remainder 10 3); => 1
(= 1 1)        ; => #t
(and #t #f)    ; => #f
```

### 変数・関数定義
`define`を使用して変数や関数を定義します:

```scheme
; 変数定義
(define x 10)
(define y (+ x 5))

; 関数定義
(define (square x)
  (* x x))
(square 4)  ; => 16

; lambda式による関数定義
(define sum-of-squares
  (lambda (x y)
    (+ (square x) (square y))))
```

### 制御構造
条件分岐とループ:

```scheme
; if式
(if (> x 0)
    "positive"
    "negative")

; cond式
(cond
  ((< x 0) "negative")
  ((> x 0) "positive")
  (else "zero"))

; let式による局所変数
(let ((x 1)
      (y 2))
  (+ x y))
```

### リスト操作
- `car`: リストの先頭要素を取得
- `cdr`: リストの先頭以外の要素を取得
- `cons`: 要素をリストの先頭に追加
- `list`: リストを生成
- `null?`: 空リストかどうかを判定


```scheme
(car '(1 2 3))     ; => 1
(cdr '(1 2 3))     ; => (2 3)
(cons 1 '(2 3))    ; => (1 2 3)
(list 1 2 3)       ; => (1 2 3)
(null? '())        ; => #t
```


## 実装予定の機能

- [x] [R5RS準拠](https://www.unixuser.org/~euske/doc/r5rs-ja/r5rs-ja.pdf)の基本データ型 (整数と真偽値のサポート)
- [x] 数値演算（整数のみ）
- [x] 変数定義
- [x] 関数定義
- [ ] 制御構造（if, cond, let等）
- [ ] リスト操作と再帰
- [ ] 文字列操作
- [x] エラーハンドリング (anyhow)
- [x] REPL (対話型実行環境)
- [ ] マクロシステム
