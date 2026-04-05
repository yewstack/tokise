use std::io::Result;
use std::marker::PhantomData;

/// Spawns a task on current thread.
///
/// # Panics
///
/// This function will panic when not being executed from within the [Runtime].
#[inline(always)]
pub fn spawn_local<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    crate::imp::spawn_local(f);
}

/// A Runtime Builder.
#[derive(Debug)]
pub struct RuntimeBuilder {
    worker_threads: usize,
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self {
            worker_threads: crate::imp::get_default_runtime_size(),
        }
    }
}

impl RuntimeBuilder {
    /// Creates a new Runtime Builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of worker threads the Runtime will use.
    ///
    /// # Default
    ///
    /// The default number of worker threads is the number of available logical CPU cores.
    ///
    /// # Note
    ///
    /// This setting has no effect if current platform has no thread support (e.g.: WebAssembly).
    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        self.worker_threads = val;

        self
    }

    /// Creates a Runtime.
    pub fn build(&mut self) -> Result<Runtime> {
        Ok(Runtime {
            inner: crate::imp::Runtime::new(self.worker_threads)?,
        })
    }
}

/// An asynchronous Runtime.
#[derive(Debug, Clone, Default)]
pub struct Runtime {
    inner: crate::imp::Runtime,
}

impl Runtime {
    /// Creates a runtime Builder.
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Spawns a task with it pinned to a worker thread.
    ///
    /// This can be used to execute non-Send futures without blocking the current thread.
    ///
    /// [`spawn_local`] is available with tasks executed with `spawn_pinned`.
    #[inline(always)]
    pub fn spawn_pinned<F, Fut>(&self, create_task: F)
    where
        F: FnOnce() -> Fut,
        F: Send + 'static,
        Fut: Future<Output = ()> + 'static,
    {
        self.inner.spawn_pinned(create_task);
    }
}

/// A Local Runtime Handle.
///
/// This type can be used to acquire a runtime handle to spawn local tasks.
#[derive(Debug, Clone)]
pub struct LocalHandle {
    inner: crate::imp::LocalHandle,
    // This type is not send or sync.
    _marker: PhantomData<*const ()>,
}

impl LocalHandle {
    /// Creates a Handle to current Runtime worker.
    ///
    /// # Panics
    ///
    /// This method will panic if not called from within the [Runtime].
    pub fn current() -> Self {
        let inner = crate::imp::LocalHandle::current();

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Creates a Handle to current Runtime worker.
    ///
    /// This methods will return `None` if called from outside the [Runtime].
    pub fn try_current() -> Option<Self> {
        let inner = crate::imp::LocalHandle::try_current()?;

        Some(Self {
            inner,
            _marker: PhantomData,
        })
    }

    /// Spawns a Future with current [Runtime] worker.
    #[inline(always)]
    pub fn spawn_local<F>(&self, f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.inner.spawn_local(f);
    }
}
