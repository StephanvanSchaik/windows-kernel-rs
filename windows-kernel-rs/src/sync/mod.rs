pub mod fast_mutex;
pub mod push_lock;

pub use self::fast_mutex::FastMutex as Mutex;
pub use self::push_lock::PushLock as RwLock;
