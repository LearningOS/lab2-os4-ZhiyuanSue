# lab5实验报告

使用时间：约1天左右

## 实现的简单描述

在sync文件夹下我增加了一个DL_detect模块用于包涵死锁检测的代码

互斥锁可看作信号量数量等于1的特例，因此，以下描述仅仅只说信号量了。

向量 Available：是目前所有的信号量的数量构成的一个向量

Allocation矩阵：每个线程所占用的各种信号量的数量构成的矩阵

需求矩阵 Need：需求矩阵一定是每个线程，在需求的那个信号量上面有1，其他全都是0，这样构成的一个矩阵。 

除此之外，在增加线程的时候需要增加矩阵的行，线程退出的时候需要把资源全都还回去。



## 问答作业

1. 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 - 需要回收的资源有哪些？ - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

   解答：

   需要回收的资源有：本身占用的内存，也就是memset，子进程child的内容，文件描述符表，其他线程

   其他线程的taskcontrolblock可能在锁部分被引用，不需要回收，因为编译器会回收掉。

2. 对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？

```
 1impl Mutex for Mutex1 {
 2    fn unlock(&self) {
 3        let mut mutex_inner = self.inner.exclusive_access();
 4        assert!(mutex_inner.locked);
 5        mutex_inner.locked = false;
 6        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
 7            add_task(waking_task);
 8        }
 9    }
10}
11
12impl Mutex for Mutex2 {
13    fn unlock(&self) {
14        let mut mutex_inner = self.inner.exclusive_access();
15        assert!(mutex_inner.locked);
16        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
17            add_task(waking_task);
18        } else {
19            mutex_inner.locked = false;
20        }
21    }
22}
```

​			解答：

区别是最后多了那个else语句。问题在于，由于没有找到等待的task，所以没释放锁，因此会有可能卡死在这里。

## 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方，欢迎畅所欲言。

我在实现allocation矩阵的时候，发现要实现一个矩阵的访问，但是，非常不幸的是，我搞不定，希望能够提供一个矩阵库。（我用VEC<VEC<>>，但是总有一堆错误。但凡这是个C。。。）

最后我不得不面向测例编程，开个数组完事儿。
