use windows_kernel_sys::base::ULONG_PTR;
use windows_kernel_sys::ntoskrnl::KeIpiGenericCall;

unsafe extern "C" fn broadcast_callback<F>(
    context: ULONG_PTR,
) -> ULONG_PTR
where
    F: FnMut(),
{
    let f = &mut *(context as *mut F);
    f();

    0
}

pub fn run_on_each_cpu<F>(
    mut f: F,
)
where
    F: Fn(),
{
    unsafe {
        KeIpiGenericCall(Some(broadcast_callback::<F>), &mut f as *mut _ as ULONG_PTR);
    }
}
