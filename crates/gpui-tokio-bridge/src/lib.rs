use std::future::Future;

use gpui::{App, AsyncApp, Global, Task};

/// Starts a small multi-threaded Tokio runtime and stores its handle as a GPUI global.
/// Call this once, before spawning anything that needs Tokio (e.g. `reqwest` calls).
pub fn init(cx: &mut App) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("failed to start Tokio runtime");

    let handle = runtime.handle().clone();
    cx.set_global(GlobalTokio {
        _runtime: runtime,
        handle,
    });
}

struct GlobalTokio {
    // Held only to keep the runtime (and its worker threads) alive.
    _runtime: tokio::runtime::Runtime,
    handle: tokio::runtime::Handle,
}

impl Global for GlobalTokio {}

/// Runs `future` on the Tokio runtime started by [`init`], and returns its result as a GPUI
/// [`Task`] you can `.await` from a `cx.spawn` block.
pub fn spawn<Fut, R>(cx: &AsyncApp, future: Fut) -> Task<Result<R, tokio::task::JoinError>>
where
    Fut: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let handle = cx.read_global(|tokio: &GlobalTokio, _| tokio.handle.clone());

    let join_handle = handle.spawn(future);
    let abort_handle = join_handle.abort_handle();

    cx.background_executor().spawn(async move {
        struct AbortOnDrop(tokio::task::AbortHandle);
        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                self.0.abort();
            }
        }
        let _abort_on_drop = AbortOnDrop(abort_handle);

        join_handle.await
    })
}
