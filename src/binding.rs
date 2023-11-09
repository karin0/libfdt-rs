use crate::ffi::*;
use crate::*;

use alloc::borrow::{Borrow, BorrowMut, ToOwned};
use alloc::vec::Vec;
use core::ffi::CStr;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
#[repr(transparent)]
pub struct Fdt;

#[derive(Debug, Clone)]
pub struct FdtBuf {
    inner: Vec<u8>,
}

const FDT_V1_SIZE: usize = 7 * 4;

fn validate_bytes(data: &[u8]) -> Result<&[u8]> {
    if data.len() < FDT_V1_SIZE {
        return Err(Error::Invalid);
    }
    let len: usize = unsafe { crate::fdt_size(data.as_ptr() as *mut c_void) }.try_into()?;
    if len <= data.len() {
        Ok(data)
    } else {
        Err(Error::Invalid)
    }
}

impl Fdt {
    pub fn from_bytes(data: &[u8]) -> Result<&Self> {
        validate_bytes(data)?;
        Ok(unsafe { Self::from_bytes_unchecked(data) })
    }

    pub fn from_bytes_mut(data: &mut [u8]) -> Result<&mut Self> {
        validate_bytes(data)?;
        Ok(unsafe { Self::from_bytes_mut_unchecked(data) })
    }

    pub unsafe fn from_bytes_unchecked(data: &[u8]) -> &Self {
        &*(data as *const [u8] as *const Self)
    }

    pub unsafe fn from_bytes_mut_unchecked(data: &mut [u8]) -> &mut Self {
        &mut *(data as *mut [u8] as *mut Self)
    }

    pub unsafe fn from_ptr<'a>(data: *const u8) -> &'a Self {
        &*(data as *const Self)
    }

    pub unsafe fn from_mut_ptr<'a>(data: *mut u8) -> &'a mut Self {
        &mut *(data as *mut Self)
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut Self as *mut u8
    }

    fn as_raw_ptr(&self) -> *const c_void {
        self as *const Self as *const c_void
    }

    fn as_raw_mut_ptr(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }

    pub fn len(&self) -> usize {
        (unsafe { crate::fdt_size(self.as_raw_ptr() as *mut c_void) }) as usize
    }

    pub fn pack(&mut self) -> Result<()> {
        wrap_unit(unsafe { crate::fdt_pack(self.as_raw_mut_ptr()) })
    }

    pub fn overlay_apply(&mut self, overlay: &mut Fdt) -> Result<()> {
        wrap_unit(unsafe {
            crate::fdt_overlay_apply(self.as_raw_mut_ptr(), overlay.as_raw_mut_ptr())
        })
    }

    pub fn remove_node(&mut self, path: &CStr) -> Result<()> {
        wrap_unit(unsafe {
            crate::fdt_remove_node(self.as_raw_mut_ptr(), path.as_ptr() as *const c_char)
        })
    }
}

impl Deref for FdtBuf {
    type Target = Fdt;

    fn deref(&self) -> &Self::Target {
        unsafe { Fdt::from_bytes_unchecked(&self.inner) }
    }
}

impl DerefMut for FdtBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Fdt::from_bytes_mut_unchecked(&mut self.inner) }
    }
}

impl Borrow<Fdt> for FdtBuf {
    fn borrow(&self) -> &Fdt {
        self
    }
}

impl BorrowMut<Fdt> for FdtBuf {
    fn borrow_mut(&mut self) -> &mut Fdt {
        self
    }
}

impl ToOwned for Fdt {
    type Owned = FdtBuf;

    fn to_owned(&self) -> Self::Owned {
        FdtBuf::from_fdt(self)
    }
}

impl FdtBuf {
    pub fn from_fdt(fdt: &Fdt) -> Self {
        Self {
            inner: fdt.as_bytes().to_vec(),
        }
    }

    pub fn from_fdt_capacity(fdt: &Fdt, cap: usize) -> Result<Self> {
        let mut buf = Vec::with_capacity(cap);
        wrap_unit(unsafe {
            crate::fdt_open_into(
                fdt.as_raw_ptr(),
                buf.as_mut_ptr() as *mut c_void,
                cap.try_into()?,
            )
        })?;
        Ok(Self { inner: buf })
    }

    pub fn reserve(&mut self, additional: usize) -> Result<()> {
        let cap = self.len() + additional;
        let new = Self::from_fdt_capacity(self, cap)?;
        *self = new;
        Ok(())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}
