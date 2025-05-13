# lab1

## 编程作业

实现了`sys_trace()`调用的三种功能：

当入参`trace_request`值为`0`时，通过类型转换将`usize`的`id`转化为`*const u8`，
然后返回读取结果。

当入参`trace_request`值为`1`时，通过类型转换将`usize`的`id`转化为`*const u8`，
再将`usize`的`data`转化为`u8`，然后将数据写入指定的地址处。

当入参`trace_request`值为`3`时，需要打印当前任务对指定的系统调用的调用次数。
因为不能使用标准库里的`HashMap`，所以在`TaskControlBlock`结构体中新增`u32`数组`syscall_count: [u32; MAX_SYSCALL_ID + 1]`来记录各个系统调用的调用次数，同时提供`increase_syscall_count()`和`get_syscall_count()`两个方法来根据`syscall_id`来写和读对应调用的调用次数。在`TaskManager`中封装上述两个方法，使外部函数可以通过`TASK_MANAGER`这个全局实例，通过其中的`current_task`字段拿到当前任务的`TaskControlBlock`，进而能够通过上面方法来修改和获取次数信息。最后，在系统调用的入口`syscall()`处增加计数，然后在`sys_trace()`处读取计数。

## 简答作业