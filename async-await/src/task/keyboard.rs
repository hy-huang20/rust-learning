use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;

/// 全局扫描码队列
/// 这里 ArrayQueue 是无锁（lock-free）并发队列
/// 虽然都允许延迟初始化，但 uninit() 允许开发者
/// 手动决定初始化的时机；而 lazy_static! 宏必须
/// 首次访问时初始化，所以如果不幸地在中断处理程序
/// 中被首次访问到并初始化，从而在中断处理程序中执
/// 行堆分配，则在中断处理程序中引入了较大时间开销
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>>
    = OnceCell::uninit();

/// Called by the keyboard interrupt handler
/// 
/// Must not block or allocate
/// pub(crate) 表示 仅当前 crate 可见
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; 
                dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode 
            queue uninitialized");
    }
}