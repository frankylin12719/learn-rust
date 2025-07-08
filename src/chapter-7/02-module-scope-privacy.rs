fn main() {
    println!("Crate包含模块Modules。");
    // use 关键字
    println!("  use 关键字
    ○ 使用use关键字将路径引入作用域（形成软连接），减少编写路径带来的繁琐工作
    ○ 常用的use路径一般指定到mod即可，增加函数清晰度
    ○ use 引入结构体、枚举等数据类型时，习惯于指定到完整路径
    ○ 可用过as为use引入的模块、结构体等重新命名
    ○ 使用pub use重导出(re-exporting)名称，类似于re-writing path name（重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它就定义在这个新位置一样）
    ○ 嵌套路径合并use列表
      // use std::io;
      // use std::cmp::Ordering;
      // ==>> use std::\"{{cmp::Ordering, io}}\";

      // use std::io::Write;
      // use std::io;
      // ==>> use std::io::\"{{self, Write}}\";
    ○ glob运算符 *，常用于测试模块
      // use std::collections::*;
      // 将std::collections的所有公有项引入当前作用域
    ");
}
