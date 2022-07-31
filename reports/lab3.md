# lab3实验报告

## spawn创建新进程

spawn是fork+exec，把这两部分的代码合起来即可。区别是，一些数据结构，在fork里面初始化之后，还要在exec里面更改，但是在spawn里面，一步到位，即可。

## stride调度算法

算法大概就是task里面加了pass，stride和priority三个字段。

初始化的时候，stride是bigstride去除以priority，其他和任务说明一样。

更改优先级的时候需要同时更改stride。

调度的时候，遍历找出pass最小的，把找出来的那个加上一个stride，并从vec中删除掉。



## 问答题

stride 算法深入

> stride 算法原理非常简单，但是有一个比较大的问题。例如两个 stride = 10 的进程，使用 8bit 无符号整形储存 pass， p1.pass = 255, p2.pass = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。
>
> - 实际情况是轮到 p1 执行吗？为什么？
>
>   解答：实际情况不是轮到p1执行，原因是250+10=260，8位溢出变成4，小于255，最终是p2执行。
>
> 我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 PASS_MAX – PASS_MIN <= BigStride / 2。
>
> - 为什么？尝试简单说明（不要求严格证明）。
>
>   步长小的执行的次数多，而步长大的执行的次数少。总而言之，两者差的不会特别多。
>
> - 已知以上结论，**考虑溢出的情况下**，可以为 pass 设计特别的比较器，让 BinaryHeap<Pass> 的 pop 方法能返回真正最小的 Pass。补全下列代码中的 `partial_cmp` 函数，假设两个 Pass 永远不会相等。
>
> ```
> use core::cmp::Ordering;
> 
> struct Pass(u64);
> 
> impl PartialOrd for Pass {
>     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
>         // 
>         let cmp=(self.0-other.0) as i64
>         if cmp<0
>         {
>         		return Some(Ordering::Less);
>         }
>         else if cmp==0
>         {
>         		return Some(Ordering::Equal);
>         }
>         else if cmp>0
>         {
>         		return Some(Ordering::Greater);
>         }
>         None
>     }
> }
> 
> impl PartialEq for Pass {
>     fn eq(&self, other: &Self) -> bool {
>         false
>     }
> }
> ```
>
> TIPS: 使用 8 bits 存储 pass, BigStride = 255, 则: `(125 < 255) == false`, `(129 < 255) == true`.
