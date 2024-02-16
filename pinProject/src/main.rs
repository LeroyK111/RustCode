/*
! 解决异步编程的引用问题 pin 和 unpin
*/

#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    // 对于每个字段，我们需要选择是返回对该字段的未固定(&mut)引用
    // 还是固定(Pin<&mut >)引用。
    // 默认情况下，它是未固定的
    start: Option<Instant>,
    // 此属性选择固定引用
    #[pin]
    future: Fut,
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // 这将返回一个具有所有相同字段的类型，所有相同的类型，
        // 除了用#[pin]定义的字段将被固定。
        let mut this = self.project();

        // 调用内部poll，测量花了多长时间。
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // 内部Future需要更多的时间，所以这个Future也需要更多的时间
            Poll::Pending => Poll::Pending,
            // Success!
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}


// 最后，我们的目标完成了——我们在没有任何不安全代码的情况下完成了这一切。
fn main() {
    println!("Hello, world!");
}
