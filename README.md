# 用于展示如何在Rust中处理Web界面的简易计算器。

![Screenshot7](tutorial/img/Screenshot7.png)

## 指南：[这里](tutorial/tutorial.md)。

## 演示：[这里](https://mae664128.github.io/calculator-wasm-rust-pwa/)。

这是一个编写[PWA](https://en.wikipedia.org/wiki/Progressive_web_app)应用程序的示例，可在浏览器和Windows操作系统计算机上使用。使用了Rust编程语言和eframe框架（[egui](https://github.com/emilk/egui#quick-start)）。完成的应用程序可作为Windows操作系统的可执行文件，也可作为Webassembly文件使用。在此过程中，使用了GitHub Action来监视代码编写的正确性，并构建可执行程序文件，同时部署了（使用Webassembly的程序版本）作为网页（GitHub Pages）。