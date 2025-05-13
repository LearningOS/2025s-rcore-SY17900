# lab3

## 编程作业

迁移了`sys_get_time`、`sys_mmap`以及`sys_munmap`系统调用。

完成`sys_spawn`系统调用：为`TCB`添加`spawn`方法，为子进程创建新的虚拟地址空间，并进行`pid`和内核栈的分配，随后创建子进程的`TCB`，设置好`trap`上下文，并将`TCB`添加到就绪队列中。

实现`stride`调度算法：为`TaskControlBlockInner`添加`stride`和`priority`字段。每次通过`TaskManager::fetch()`寻找下一个可执行进程时，遍历就绪队列，找到`stride`最小的进程。每次完成一个进程的阶段性执行并通过`TaskManager::add()`将其添加到就绪队列时，通过`TaskControlBlockInner::add_stride()`递增`stride`。

## 简答作业