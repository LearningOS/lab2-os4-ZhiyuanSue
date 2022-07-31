# lab4实验报告

## **linkat**和**unlinkat**实现

需要根据虚拟地址指针，得到真实的string。

链接的建立和解除跟task无关

因此，需要在vfs中实现建立链接的函数，而由于单层文件树，可以直接用根节点建立和拆除

## **fstat**实现

fstat是根据文件描述符去找到对应的文件信息

对fs模块下的stdio和file分别实现fstat

最后，在内核态找到对应数据结构的物理地址，填充

## **vfs**：

由于操作系统内核和文件系统分离的设计，需要在vfs中加上一定的函数，供内核调用。

添加函数作为上述功能的接口：

get_inode

get_mode

get_nlink

add_one_nlink

delete_one_nlink

## 问答题

### Ch6:

在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

解答：因为easy-fs就一层文件，所以root inode实际上是作为整个文件系统的目录来使用的，如果root inode的内容损坏了，那就是整个文件系统目录坏掉了，相应文件就无法通过该文件系统找到了。

### Ch7:

## 问答作业[¶](https://learningos.github.io/rust-based-os-comp2022/chapter7/3exercise.html#id2)

1. 举出使用 pipe 的一个实际应用的例子。

   解答：使用管道符将grep的结果重定向

2. 如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制。

   解答：由于这个实验的页表是双页表，所以完全可以向内核申请一个共享内存页面，并添加锁进行共享访问。