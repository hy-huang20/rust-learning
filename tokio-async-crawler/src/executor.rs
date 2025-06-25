use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
    thread,
    time::Duration,
};

// 主 Executor 结构体
#[derive(Clone)]
pub struct MiniTokio {
    scheduled: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl MiniTokio {
    pub fn new() -> Self {
        Self {
            scheduled: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// 生成一个新任务
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::new(future, self.scheduled.clone());
    }

    /// 运行 executor 直到所有任务完成
    pub fn run(&self) {
        loop {
            let task = match self.scheduled.lock().unwrap().pop_front() {
                Some(task) => task,
                None => break, // 没有任务了，退出
            };

            let mut future_slot = task.future.lock().unwrap();
            
            // 获取 Future
            if let Some(mut future) = future_slot.take() {
                // 创建 Waker
                let waker = Waker::from(task.clone());
                let mut cx = Context::from_waker(&waker);
                
                // 轮询 Future
                if future.as_mut().poll(&mut cx).is_pending() {
                    // 如果 Future 还没完成，把它放回去
                    *future_slot = Some(future);
                }
            }
        }
    }
}

// 任务结构体
struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    executor: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl Task {
    fn new<F>(future: F, executor: Arc<Mutex<VecDeque<Arc<Task>>>>) -> Arc<Self>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(future))),
            executor,
        });
        // 立即调度任务
        task.schedule();
        task
    }

    // 将任务放回调度队列
    fn schedule(self: &Arc<Self>) {
        self.executor.lock().unwrap().push_back(self.clone());
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.schedule();
    }
}