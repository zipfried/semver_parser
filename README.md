# 基于 Pest 的语义化版本解析

> 参考：[合法语义化版本的巴科斯范式语法](https://semver.org/lang/zh-CN/#%E5%90%88%E6%B3%95%E8%AF%AD%E4%B9%89%E5%8C%96%E7%89%88%E6%9C%AC%E7%9A%84%E5%B7%B4%E7%A7%91%E6%96%AF%E8%8C%83%E5%BC%8F%E8%AF%AD%E6%B3%95)。

## 示例

（最后一个输入非法）

```
semver 1.0.0-alpha1+u8u8 2.3.5 01.29.10
```

输出：

```
Semver {
    major: 1,
    minor: 0,
    patch: 0,
    pre: [
        "alpha1",
    ],
    build: [
        "u8u8",
    ],
}

Semver {
    major: 2,
    minor: 3,
    patch: 5,
    pre: [],
    build: [],
}
```
