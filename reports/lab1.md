# lab1实验报告

## 实现的功能

**./src/syscall/mod.rs::syscall**

1/时间很重要，所以，需立即运行get time得到当前时间

2/每次系统调用，在task ctr block里面计数。

**./src/syscall/mod.rs::process.rs**

实现sys_task_info函数。

1/需要找当前的task ctr block，得到进程运行状态，虽然肯定是running状态。

2/对于使用的系统调用进行读取

3/获取任务第一次被调度时间然后当前时间相减即可。

**./src/task/mod.rs**

1/一些数据结构的初始化。

2/当某个任务第一次被调用时，记录下时间。置位是否被初始化调度了

**./src/tesk/task.rs**

按照描述，添加以下数据结构：

任务第一次被调度的时刻

系统调用次数向量

是否调度过了的标志位

## 简答题

1/正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 [Rust 三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rust-based-os-comp2022/tree/main/user/src/bin) ， 注意在编译时至少需要指定 `LOG=ERROR` 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

解答：

三个bad测例输出信息如下：

```
[ERROR] [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400408, core dumped.
[ERROR] [kernel] IllegalInstruction in application, core dumped.
[ERROR] [kernel] IllegalInstruction in application, core dumped.
```

sbi信息如下：

```
RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0
```

2/深入理解 [trap.S](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/os3-ref/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

   解答：

   a0代表了函数的返回值。

   两种使用场景:1/进程的切换返回用户态

2. L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

   ```
   ld t0, 32*8(sp)
   ld t1, 33*8(sp)
   ld t2, 2*8(sp)
   csrw sstatus, t0
   csrw sepc, t1
   csrw sscratch, t2
   ```

   解答：

   处理了sstatus，sepc，sscratch三个寄存器

   sstatus 记录中断信息

   sepc 返回后从哪里开始执行

   sscratch 内核栈的位置

   

3. L53-L59：为何跳过了 `x2` 和 `x4`？

   ```
   ld x1, 1*8(sp)
   ld x3, 3*8(sp)
   .set n, 5
   .rept 27
      LOAD_GP %n
      .set n, n+1
   .endr
   ```

   解答：

   x2是已经保存了

   x4用不到

4. L63：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

   ```
   csrrw sp, sscratch, sp
   ```

   解答：一个是指向内核栈，一个指向用户栈，这条指令之后两者交换

5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

   解答：

   sret指令

   执行之后根据sstatus确定返回的是用户态还是内核态并跳转到sepc指向的位置

6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

   ```
   csrrw sp, sscratch, sp
   ```

   解答：同上

7. 从 U 态进入 S 态是哪一条指令发生的？

   解答：
   
   ecall指令
