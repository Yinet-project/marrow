use core::ffi::c_void;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::option::Option::Some;
use core::cell::RefCell;
use crate::debug;

pub fn read_file_callback<F>(s: &str, mut f: F) -> usize
where
    F: FnMut(u8),
{
    #[link(wasm_import_module = "wstd")]
    extern "C" {
        fn _read_file_callback(
            cb: unsafe extern "C" fn(*mut c_void,u8),
            user_data: *mut c_void,
            path: *const u8,
            path_len: usize,
        ) -> usize;
    }

    unsafe extern "C" fn hook<F>(user_data: *mut c_void, result: u8)    where
        F: FnMut(u8),
    {
        (*(user_data as *mut F))(result)
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    let bytes = s.as_bytes();

    unsafe { _read_file_callback(hook::<F>, user_data, bytes.as_ptr(), bytes.len()) }
}


#[derive(Debug, Clone)]
pub struct ReadFile {
    inner:RefCell<Inner>
}

#[derive(Debug,Clone)]
struct Inner{
    result: Option<u8>,
    task: Option<Waker>,
}


impl Default for ReadFile {
    fn default() -> Self {
        ReadFile{
            inner: RefCell::new(Inner { result: None, task: None})
        }
    }
}

impl Future for ReadFile {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // let mut task = self.task.unwrap();
        debug::println("poll");
        let mut inner = self.inner.borrow_mut();

        if let Some(x) = inner.result{
            return Poll::Ready(x);
        }

        inner.task = Some(cx.waker().clone());
        debug::println(&alloc::format!("poll inner ptr:{:?}",self.inner.as_ptr()));
        Poll::Pending
    }
}

pub fn read_file(s: &str) -> ReadFile {
    //第一次创建任务的时候会触发一次poll
    //是回调触发后才开始poll
    //由于第一次poll执行失败，waker没传出来，导致回调了也无法唤醒第二次poll
    let fu = ReadFile::default();
    let inner = fu.inner.as_ptr();
    unsafe {
        read_file_callback(s,   move|result:u8| {
            let inner = &mut *inner;
            inner.result = Some(result);
            let task_op = &inner.task.as_ref();
            if task_op.is_some() {
                task_op.unwrap().wake_by_ref()
            };
        });
    }

    ReadFile::default()
}

#[no_mangle]
pub extern "C" fn call_read_file_callback_fn(
    cb: unsafe extern "C" fn(*mut c_void,u8),
    user_data: *mut c_void,
    result:u8
) {
    unsafe { cb(user_data,result) }
}
