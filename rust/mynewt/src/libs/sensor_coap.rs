/* automatically generated by rust-bindgen */

use
super::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
#[cfg(feature = "NOTUSED")]  ////  TODO
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::core::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
#[cfg(feature = "NOTUSED")]  ////  TODO
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const COAP_LINK_FORMAT_FILTERING: u32 = 0;
pub const COAP_PROXY_OPTION_PROCESSING: u32 = 0;
pub const COAP_MAX_ATTEMPTS: u32 = 2;
pub const COAP_OBSERVE_REFRESH_INTERVAL: u32 = 20;
pub const COAP_DEFAULT_PORT: u32 = 5683;
pub const COAP_DEFAULT_MAX_AGE: u32 = 60;
pub const COAP_RESPONSE_RANDOM_FACTOR: f64 = 1.5;
pub const COAP_MAX_RETRANSMIT: u32 = 4;
pub const COAP_HEADER_LEN: u32 = 4;
pub const COAP_TOKEN_LEN: u32 = 8;
pub const COAP_ETAG_LEN: u32 = 8;
pub const COAP_MAX_URI: u32 = 32;
pub const COAP_MAX_URI_QUERY: u32 = 32;
pub const COAP_TCP_LENGTH8_OFF: u32 = 13;
pub const COAP_TCP_LENGTH16_OFF: u32 = 269;
pub const COAP_TCP_LENGTH32_OFF: u32 = 65805;
pub const COAP_TCP_TYPE0: u32 = 0;
pub const COAP_TCP_TYPE8: u32 = 13;
pub const COAP_TCP_TYPE16: u32 = 14;
pub const COAP_TCP_TYPE32: u32 = 15;
pub const COAP_HEADER_OPTION_DELTA_MASK: u32 = 240;
pub const COAP_HEADER_OPTION_SHORT_LENGTH_MASK: u32 = 15;
pub const COAP_PORT_UNSECURED: u32 = 5683;
pub type __uint8_t = ::cty::c_uchar;
pub type __uint16_t = ::cty::c_ushort;
pub type __uint32_t = ::cty::c_ulong;
pub type __uint64_t = ::cty::c_ulonglong;
#[doc = " A mbuf pool from which to allocate mbufs. This contains a pointer to the os"]
#[doc = " mempool to allocate mbufs out of, the total number of elements in the pool,"]
#[doc = " and the amount of \"user\" data in a non-packet header mbuf. The total pool"]
#[doc = " size, in bytes, should be:"]
#[doc = "  os_mbuf_count * (omp_databuf_len + sizeof(struct os_mbuf))"]
#[repr(C)]
pub struct os_mbuf_pool {
    #[doc = " Total length of the databuf in each mbuf.  This is the size of the"]
    #[doc = " mempool block, minus the mbuf header"]
    pub omp_databuf_len: u16,
    #[doc = " The memory pool which to allocate mbufs out of"]
    pub omp_pool: *mut os_mempool,
    pub omp_next: os_mbuf_pool__bindgen_ty_1,
}
#[repr(C)]
pub struct os_mbuf_pool__bindgen_ty_1 {
    pub stqe_next: *mut os_mbuf_pool,
}
impl Default for os_mbuf_pool__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl Default for os_mbuf_pool {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = " Chained memory buffer."]
#[repr(C)]
pub struct os_mbuf {
    #[doc = " Current pointer to data in the structure"]
    pub om_data: *mut u8,
    #[doc = " Flags associated with this buffer, see OS_MBUF_F_* defintions"]
    pub om_flags: u8,
    #[doc = " Length of packet header"]
    pub om_pkthdr_len: u8,
    #[doc = " Length of data in this buffer"]
    pub om_len: u16,
    #[doc = " The mbuf pool this mbuf was allocated out of"]
    pub om_omp: *mut os_mbuf_pool,
    pub om_next: os_mbuf__bindgen_ty_1,
    #[doc = " Pointer to the beginning of the data, after this buffer"]
    pub om_databuf: __IncompleteArrayField<u8>,
}
#[repr(C)]
pub struct os_mbuf__bindgen_ty_1 {
    pub sle_next: *mut os_mbuf,
}
impl Default for os_mbuf__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl Default for os_mbuf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = " A memory block structure. This simply contains a pointer to the free list"]
#[doc = " chain and is only used when the block is on the free list. When the block"]
#[doc = " has been removed from the free list the entire memory block is usable by the"]
#[doc = " caller."]
#[repr(C)]
pub struct os_memblock {
    pub mb_next: os_memblock__bindgen_ty_1,
}
#[repr(C)]
pub struct os_memblock__bindgen_ty_1 {
    pub sle_next: *mut os_memblock,
}
impl Default for os_memblock__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl Default for os_memblock {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = " Memory pool"]
#[repr(C)]
pub struct os_mempool {
    #[doc = " Size of the memory blocks, in bytes."]
    pub mp_block_size: u32,
    #[doc = " The number of memory blocks."]
    pub mp_num_blocks: u16,
    #[doc = " The number of free blocks left"]
    pub mp_num_free: u16,
    #[doc = " The lowest number of free blocks seen"]
    pub mp_min_free: u16,
    #[doc = " Bitmap of OS_MEMPOOL_F_[...] values."]
    pub mp_flags: u8,
    #[doc = " Address of memory buffer used by pool"]
    pub mp_membuf_addr: u32,
    pub mp_list: os_mempool__bindgen_ty_1,
    pub __bindgen_anon_1: os_mempool__bindgen_ty_2,
    #[doc = " Name for memory block"]
    pub name: *mut ::cty::c_char,
}
#[repr(C)]
pub struct os_mempool__bindgen_ty_1 {
    pub stqe_next: *mut os_mempool,
}
impl Default for os_mempool__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct os_mempool__bindgen_ty_2 {
    pub slh_first: *mut os_memblock,
}
impl Default for os_mempool__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl Default for os_mempool {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed)]
#[derive(Default)]
pub struct oc_ep_hdr {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
}
impl oc_ep_hdr {
    #[inline]
    pub fn oe_type(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_oe_type(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn oe_flags(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 5u8) as u8) }
    }
    #[inline]
    pub fn set_oe_flags(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(oe_type: u8, oe_flags: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let oe_type: u8 = unsafe { ::core::mem::transmute(oe_type) };
            oe_type as u64
        });
        __bindgen_bitfield_unit.set(3usize, 5u8, {
            let oe_flags: u8 = unsafe { ::core::mem::transmute(oe_flags) };
            oe_flags as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Default)]
pub struct oc_endpoint {
    pub ep: oc_ep_hdr,
    pub _res: [u8; 23usize],
}
pub type oc_endpoint_t = oc_endpoint;
#[repr(C)]
#[derive(Default)]
pub struct stats_coap_stats {}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut coap_stats: stats_coap_stats;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut coap_error_message: *mut ::cty::c_char;
}
#[repr(C)]
#[derive(Default)]
pub struct oc_server_handle {
    pub endpoint: oc_endpoint_t,
}
#[repr(C)]
pub struct sensor_value {
    pub key: *const ::cty::c_char,
    pub val_type: ::cty::c_int,
    pub int_val: u16,
    pub float_val: f32,
}
impl Default for sensor_value {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Init the Sensor CoAP module. Called by sysinit() during startup, defined in pkg.yml."]
    pub fn init_sensor_coap();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_coap_ready() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn init_sensor_post(server: *mut oc_server_handle) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn do_sensor_post() -> bool;
}
#[repr(C)]
pub struct json_value__bindgen_ty_1 {
    pub u: __BindgenUnionField<u64>,
    pub fl: __BindgenUnionField<f32>,
    pub str: __BindgenUnionField<*mut ::cty::c_char>,
    pub composite: __BindgenUnionField<json_value__bindgen_ty_1__bindgen_ty_1>,
    pub bindgen_union_field: [u64; 2usize],
}
#[repr(C)]
pub struct json_value__bindgen_ty_1__bindgen_ty_1 {
    pub keys: *mut *mut ::cty::c_char,
    pub values: *mut *mut json_value,
}
impl Default for json_value__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl Default for json_value__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type json_write_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::cty::c_void,
        data: *mut ::cty::c_char,
        len: ::cty::c_int,
    ) -> ::cty::c_int,
>;
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut coap_json_encoder: json_encoder;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut coap_json_value: json_value;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Prepare to write a new JSON CoAP payload into the mbuf."]
    pub fn json_rep_new(m: *mut os_mbuf);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Close the current JSON CoAP payload.  Erase the JSON encoder."]
    pub fn json_rep_reset();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Finalise the payload and return the payload size."]
    pub fn json_rep_finalize() -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = " Start the JSON representation.  Assume top level is object."]
    #[doc = " ```"]
    #[doc = " --> {"]
    #[doc = " ```"]
    pub fn json_rep_start_root_object();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  End the JSON representation.  Assume top level is object."]
    #[doc = "  ```"]
    #[doc = "  {... --> {...}"]
    #[doc = "  ```"]
    pub fn json_rep_end_root_object();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut coap_json_mbuf: *mut os_mbuf;
}
