/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const MAXDRIVERNAME: u32 = 255;
pub const URB_SHORT_NOT_OK: u32 = 1;
pub const URB_ISO_ASAP: u32 = 2;
pub const URB_BULK_CONTINUATION: u32 = 4;
pub const URB_NO_FSBR: u32 = 32;
pub const URB_ZERO_PACKET: u32 = 64;
pub const URB_NO_INTERRUPT: u32 = 128;
pub const URB_TYPE_ISO: u8 = 0;
pub const URB_TYPE_INTERRUPT: u8 = 1;
pub const URB_TYPE_CONTROL: u8 = 2;
pub const URB_TYPE_BULK: u8 = 3;
pub const CAP_ZERO_PACKET: u32 = 1;
pub const CAP_BULK_CONTINUATION: u32 = 2;
pub const CAP_NO_PACKET_SIZE_LIM: u32 = 4;
pub const CAP_BULK_SCATTER_GATHER: u32 = 8;
pub const CAP_REAP_AFTER_DISCONNECT: u32 = 16;
pub const CAP_MMAP: u32 = 32;
pub const CAP_DROP_PRIVILEGES: u32 = 64;
pub const CAP_CONNINFO_EX: u32 = 128;
pub const CAP_SUSPEND: u32 = 256;
pub const DISCONNECT_CLAIM_IF_DRIVER: u32 = 1;
pub const DISCONNECT_CLAIM_EXCEPT_DRIVER: u32 = 2;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ctrltransfer {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub timeout: __u32,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ctrltransfer() {
    assert_eq!(
        ::std::mem::size_of::<ctrltransfer>(),
        24usize,
        concat!("Size of: ", stringify!(ctrltransfer))
    );
    assert_eq!(
        ::std::mem::align_of::<ctrltransfer>(),
        8usize,
        concat!("Alignment of ", stringify!(ctrltransfer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).bRequestType as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(bRequestType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).bRequest as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(bRequest)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).wValue as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(wValue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).wIndex as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(wIndex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).wLength as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(wLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).timeout as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ctrltransfer>())).data as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ctrltransfer),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bulktransfer {
    pub ep: ::std::os::raw::c_uint,
    pub len: ::std::os::raw::c_uint,
    pub timeout: ::std::os::raw::c_uint,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_bulktransfer() {
    assert_eq!(
        ::std::mem::size_of::<bulktransfer>(),
        24usize,
        concat!("Size of: ", stringify!(bulktransfer))
    );
    assert_eq!(
        ::std::mem::align_of::<bulktransfer>(),
        8usize,
        concat!("Alignment of ", stringify!(bulktransfer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bulktransfer>())).ep as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bulktransfer),
            "::",
            stringify!(ep)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bulktransfer>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bulktransfer),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bulktransfer>())).timeout as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bulktransfer),
            "::",
            stringify!(timeout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bulktransfer>())).data as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bulktransfer),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct setinterface {
    pub interface: ::std::os::raw::c_uint,
    pub altsetting: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_setinterface() {
    assert_eq!(
        ::std::mem::size_of::<setinterface>(),
        8usize,
        concat!("Size of: ", stringify!(setinterface))
    );
    assert_eq!(
        ::std::mem::align_of::<setinterface>(),
        4usize,
        concat!("Alignment of ", stringify!(setinterface))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setinterface>())).interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(setinterface),
            "::",
            stringify!(interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<setinterface>())).altsetting as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(setinterface),
            "::",
            stringify!(altsetting)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct disconnectsignal {
    pub signr: ::std::os::raw::c_uint,
    pub context: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_disconnectsignal() {
    assert_eq!(
        ::std::mem::size_of::<disconnectsignal>(),
        16usize,
        concat!("Size of: ", stringify!(disconnectsignal))
    );
    assert_eq!(
        ::std::mem::align_of::<disconnectsignal>(),
        8usize,
        concat!("Alignment of ", stringify!(disconnectsignal))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<disconnectsignal>())).signr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(disconnectsignal),
            "::",
            stringify!(signr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<disconnectsignal>())).context as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(disconnectsignal),
            "::",
            stringify!(context)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct getdriver {
    pub interface: ::std::os::raw::c_uint,
    pub driver: [::std::os::raw::c_char; 256usize],
}
#[test]
fn bindgen_test_layout_getdriver() {
    assert_eq!(
        ::std::mem::size_of::<getdriver>(),
        260usize,
        concat!("Size of: ", stringify!(getdriver))
    );
    assert_eq!(
        ::std::mem::align_of::<getdriver>(),
        4usize,
        concat!("Alignment of ", stringify!(getdriver))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<getdriver>())).interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(getdriver),
            "::",
            stringify!(interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<getdriver>())).driver as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(getdriver),
            "::",
            stringify!(driver)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct connectinfo {
    pub devnum: ::std::os::raw::c_uint,
    pub slow: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_connectinfo() {
    assert_eq!(
        ::std::mem::size_of::<connectinfo>(),
        8usize,
        concat!("Size of: ", stringify!(connectinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<connectinfo>(),
        4usize,
        concat!("Alignment of ", stringify!(connectinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<connectinfo>())).devnum as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(connectinfo),
            "::",
            stringify!(devnum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<connectinfo>())).slow as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(connectinfo),
            "::",
            stringify!(slow)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct conninfo_ex {
    pub size: __u32,
    pub busnum: __u32,
    pub devnum: __u32,
    pub speed: __u32,
    pub num_ports: __u8,
    pub ports: [__u8; 7usize],
}
#[test]
fn bindgen_test_layout_conninfo_ex() {
    assert_eq!(
        ::std::mem::size_of::<conninfo_ex>(),
        24usize,
        concat!("Size of: ", stringify!(conninfo_ex))
    );
    assert_eq!(
        ::std::mem::align_of::<conninfo_ex>(),
        4usize,
        concat!("Alignment of ", stringify!(conninfo_ex))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).busnum as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(busnum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).devnum as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(devnum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).speed as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(speed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).num_ports as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(num_ports)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<conninfo_ex>())).ports as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(conninfo_ex),
            "::",
            stringify!(ports)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iso_packet_desc {
    pub length: ::std::os::raw::c_uint,
    pub actual_length: ::std::os::raw::c_uint,
    pub status: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_iso_packet_desc() {
    assert_eq!(
        ::std::mem::size_of::<iso_packet_desc>(),
        12usize,
        concat!("Size of: ", stringify!(iso_packet_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<iso_packet_desc>(),
        4usize,
        concat!("Alignment of ", stringify!(iso_packet_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iso_packet_desc>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(iso_packet_desc),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iso_packet_desc>())).actual_length as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(iso_packet_desc),
            "::",
            stringify!(actual_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iso_packet_desc>())).status as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(iso_packet_desc),
            "::",
            stringify!(status)
        )
    );
}
#[repr(C)]
pub struct urb {
    pub type_: ::std::os::raw::c_uchar,
    pub endpoint: ::std::os::raw::c_uchar,
    pub status: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_uint,
    pub buffer: *mut ::std::os::raw::c_void,
    pub buffer_length: ::std::os::raw::c_int,
    pub actual_length: ::std::os::raw::c_int,
    pub start_frame: ::std::os::raw::c_int,
    pub __bindgen_anon_1: urb__bindgen_ty_1,
    pub error_count: ::std::os::raw::c_int,
    pub signr: ::std::os::raw::c_uint,
    pub usercontext: *mut ::std::os::raw::c_void,
    pub iso_frame_desc: __IncompleteArrayField<iso_packet_desc>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union urb__bindgen_ty_1 {
    pub number_of_packets: ::std::os::raw::c_int,
    pub stream_id: ::std::os::raw::c_uint,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_urb__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<urb__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(urb__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<urb__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(urb__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<urb__bindgen_ty_1>())).number_of_packets as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(urb__bindgen_ty_1),
            "::",
            stringify!(number_of_packets)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb__bindgen_ty_1>())).stream_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(urb__bindgen_ty_1),
            "::",
            stringify!(stream_id)
        )
    );
}
#[test]
fn bindgen_test_layout_urb() {
    assert_eq!(
        ::std::mem::size_of::<urb>(),
        56usize,
        concat!("Size of: ", stringify!(urb))
    );
    assert_eq!(
        ::std::mem::align_of::<urb>(),
        8usize,
        concat!("Alignment of ", stringify!(urb))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).endpoint as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(endpoint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).status as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).flags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).buffer as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).buffer_length as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(buffer_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).actual_length as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(actual_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).start_frame as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(start_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).error_count as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(error_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).signr as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(signr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).usercontext as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(usercontext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<urb>())).iso_frame_desc as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(urb),
            "::",
            stringify!(iso_frame_desc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ioctl {
    pub ifno: ::std::os::raw::c_int,
    pub ioctl_code: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ioctl() {
    assert_eq!(
        ::std::mem::size_of::<ioctl>(),
        16usize,
        concat!("Size of: ", stringify!(ioctl))
    );
    assert_eq!(
        ::std::mem::align_of::<ioctl>(),
        8usize,
        concat!("Alignment of ", stringify!(ioctl))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ioctl>())).ifno as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ioctl),
            "::",
            stringify!(ifno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ioctl>())).ioctl_code as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ioctl),
            "::",
            stringify!(ioctl_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ioctl>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ioctl),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hub_portinfo {
    pub nports: ::std::os::raw::c_char,
    pub port: [::std::os::raw::c_char; 127usize],
}
#[test]
fn bindgen_test_layout_hub_portinfo() {
    assert_eq!(
        ::std::mem::size_of::<hub_portinfo>(),
        128usize,
        concat!("Size of: ", stringify!(hub_portinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<hub_portinfo>(),
        1usize,
        concat!("Alignment of ", stringify!(hub_portinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hub_portinfo>())).nports as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hub_portinfo),
            "::",
            stringify!(nports)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hub_portinfo>())).port as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(hub_portinfo),
            "::",
            stringify!(port)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct disconnect_claim {
    pub interface: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub driver: [::std::os::raw::c_char; 256usize],
}
#[test]
fn bindgen_test_layout_disconnect_claim() {
    assert_eq!(
        ::std::mem::size_of::<disconnect_claim>(),
        264usize,
        concat!("Size of: ", stringify!(disconnect_claim))
    );
    assert_eq!(
        ::std::mem::align_of::<disconnect_claim>(),
        4usize,
        concat!("Alignment of ", stringify!(disconnect_claim))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<disconnect_claim>())).interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(disconnect_claim),
            "::",
            stringify!(interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<disconnect_claim>())).flags as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(disconnect_claim),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<disconnect_claim>())).driver as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(disconnect_claim),
            "::",
            stringify!(driver)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct streams {
    pub num_streams: ::std::os::raw::c_uint,
    pub num_eps: ::std::os::raw::c_uint,
    pub eps: __IncompleteArrayField<::std::os::raw::c_uchar>,
}
#[test]
fn bindgen_test_layout_streams() {
    assert_eq!(
        ::std::mem::size_of::<streams>(),
        8usize,
        concat!("Size of: ", stringify!(streams))
    );
    assert_eq!(
        ::std::mem::align_of::<streams>(),
        4usize,
        concat!("Alignment of ", stringify!(streams))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<streams>())).num_streams as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(streams),
            "::",
            stringify!(num_streams)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<streams>())).num_eps as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(streams),
            "::",
            stringify!(num_eps)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<streams>())).eps as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(streams),
            "::",
            stringify!(eps)
        )
    );
}
