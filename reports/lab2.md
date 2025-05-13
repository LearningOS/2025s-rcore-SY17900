# lab2

## 编程作业

通过`unsafe`的`copy_from_slice()`和指针加操作实现了`sys_get_time()`系统调用。

通过在`os/src/mm`中添加`get_u8_by_va()`和`get_u8_mutable_by_va()`函数实现从任意虚拟地址获得对应物理地址处的`u8`引用，然后据此完成了`sys_trace`系统调用。

给`MemorySet`添加`check_and_insert_framed_are()`，将一段未被映射过的虚拟地址分配某段物理地址并映射过去；实现`delete_framed_area`将一段被映射过的虚拟地址解除映射。据此实现`sys_mmap`和`sys_munmap`系统调用。

## 简答作业