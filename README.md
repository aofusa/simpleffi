Simple FFI
=====


FFIインターフェースの検証プログラム


以下の動作を確認する
- 値渡し
- 配列ポインタ
- 構造体ポインタ
- メモリアロケーション

ライブラリテスト
-----

```sh
cargo test
```

ライブラリビルド
----

```sh
# soライブラリのビルド(Pythonなどで利用)
cargo build --release --target-dir ./target/

# WebAssemblyのビルド
cargo build --release --target=wasm32-unknown-unknown --target-dir ./target/
```

実行
-----

python
```python
from ctypes import *
mylib = cdll.LoadLibrary("./target/release/libsimpleffi.so")

# 値渡し
assert mylib.simple() == 42
assert mylib.add(1, 2) == 3

# 配列ポインタ
arr = (c_int * 3)(*[1, 2, 3])

mylib.array_add(byref(arr), 3)
assert arr[0] == 2
assert arr[1] == 3
assert arr[2] == 4

# 構造体ポインタ
class Point(Structure):
    _fields_ = [
            ("x", c_int),
            ("y", c_int),
        ]

point = Point(0, 1)

mylib.struct_add(byref(point))
assert point.x == 1
assert point.y == 2

# メモリ操作
p_mem = (c_void_p)(mylib.memalloc(4 * 3))
mem = cast(p_mem, POINTER(c_int * 3))[0]
mem[0] = 1
mem[1] = 2
mem[2] = 3

mylib.array_add(mem, 3)
assert mem[0] == 2
assert mem[1] == 3
assert mem[2] == 4

memset(p_mem, 42, 1)
assert mem[0] == 42

mylib.memfree(mem, 4 * 3)
```

nodejs
```javascript
/*
// ブラウザの場合
const response = await fetch("./target/wasm32-unknown-unknown/release/simpleffi.wasm");
const buf = await response.arrayBuffer();
*/
const { default: fs } = await import("node:fs/promises");
const buf = await fs.readFile("./target/wasm32-unknown-unknown/release/simpleffi.wasm");

const { instance } = await WebAssembly.instantiate(buf);

// 値渡し
assert(instance.exports.simple() === 42);
assert(instance.exports.add(1, 2) === 3);

// 配列ポインタ
const length = 3;
const ptr = instance.exports.memalloc(
    Int32Array.BYTES_PER_ELEMENT * length
);
const arr = new Int32Array(
    instance.exports.memory.buffer,
    ptr,
    length
);
arr[0] = 1;
arr[1] = 2;
arr[2] = 3;

instance.exports.array_add(ptr, length);
assert(arr[0] === 2);
assert(arr[1] === 3);
assert(arr[2] === 4);

instance.exports.memfree(
    ptr,
    Int32Array.BYTES_PER_ELEMENT * length
);

// 構造体ポインタ
/*
type Point = {
    x: number,  // int32
    y: number,  // int32
}
*/
const stPtr = instance.exports.memalloc(
    Int32Array.BYTES_PER_ELEMENT +  // x要素
    Int32Array.BYTES_PER_ELEMENT  // y要素
);
const stViewX = new Int32Array(
    instance.exports.memory.buffer,
    stPtr + Int32Array.BYTES_PER_ELEMENT * 0,
    1
);
const stViewY = new Int32Array(
    instance.exports.memory.buffer,
    stPtr + Int32Array.BYTES_PER_ELEMENT * 1,
    1
);

let point = {
    x: stViewX,
    y: stViewY,
};
point.x[0] = 0;
point.y[0] = 1;

instance.exports.struct_add(stPtr);
assert(point.x[0] === 1);
assert(point.y[0] === 2);

instance.exports.memfree(
    stPtr,
    Int32Array.BYTES_PER_ELEMENT + Int32Array.BYTES_PER_ELEMENT
);
```

参考
-----
- [ctypes - python.org](https://docs.python.org/ja/3/library/ctypes.html)
- [構造体とメモリレイアウト - webgpufundamentals.org](https://webgpufundamentals.org/webgpu/lessons/ja/webgpu-memory-layout.html)
- 以前作成したもの https://github.com/aofusa/play-rust-ffi

