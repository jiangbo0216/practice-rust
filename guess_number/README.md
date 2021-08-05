## [crates](https://crates.io/)

cargo 会从上面的网址下载crate, cargo 首先把网址的最新的索引下载下来, 然后会检查 dependency, 把未下载的依赖库下来, 如果依赖库还依赖其他的库, 也会一并下载下来.

cargo build 会编译依赖项和源代码, 产生最终的可执行文件

cargo 提供了一套机制保证构建的结果是可以重现的, 也就是 Cargo.lock. 当存在 lock 文件, build 的时候, cargo 会使用 lock 文件指定的依赖的版本, 除非你重新指定依赖的版本

我们可以使用 cargo update, 这样会忽略 lock 文件中的内容, 同时更新 crates 的索引, 通过这个索引来找到符合我们在 Cargo.toml 文件中指定的依赖的最新版本下载下来, 同时写入 lock 文件