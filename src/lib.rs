//  Copyright (C) 2015 Steven Allen
//
//  This library is free software; you can redistribute it and/or
//  modify it under the terms of the GNU Lesser General Public
//  License as published by the Free Software Foundation; either
//  version 2.1 of the License, or (at your option) any later version.
//
//  This library is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  Lesser General Public License for more details.
//
//  You should have received a copy of the GNU Lesser General Public
//  License along with this library; if not, write to the Free Software
//  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//
//  This file is heavily based on the C header sys/acl.h
//  (C) 1999 Andreas Gruenbacher, <a.gruenbacher@computer.org>
#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_uint, c_int, c_char, ssize_t};

pub type acl_t = *mut c_void;
pub type acl_permset_t = *mut c_void;
pub type acl_entry_t = *mut c_void;

pub type acl_type_t = c_uint;
pub type acl_tag_t = c_int;
pub type acl_perm_t = c_uint;

pub const ACL_TYPE_ACCESS: acl_type_t = 0x8000;
pub const ACL_TYPE_DEFAULT: acl_type_t = 0x4000;

pub const ACL_READ: acl_perm_t = 0x04;
pub const ACL_WRITE: acl_perm_t = 0x02;
pub const ACL_EXECUTE: acl_perm_t = 0x01;

pub const ACL_UNDEFINED_TAG: acl_tag_t = 0x00;
pub const ACL_USER_OBJ: acl_tag_t = 0x01;
pub const ACL_USER: acl_tag_t = 0x02;
pub const ACL_GROUP_OBJ: acl_tag_t = 0x04;
pub const ACL_GROUP: acl_tag_t = 0x08;
pub const ACL_MASK: acl_tag_t = 0x10;
pub const ACL_OTHER: acl_tag_t = 0x20;

pub const ACL_FIRST_ENTRY: c_int = 0;
pub const ACL_NEXT_ENTRY: c_int = 1;

// On Linux link to libacl, other OSes have this in libc and need no annotation
#[cfg_attr(target_os = "linux", link(name = "acl"))]
extern "C" {
    /*=== ACL manipulation ===*/

    pub fn acl_init(count: c_int) -> acl_t;
    pub fn acl_dup(acl: acl_t) -> acl_t;
    pub fn acl_free(data: *mut c_void) -> c_int;
    pub fn acl_valid(acl: acl_t) -> c_int;

    /*=== Entry manipulation ===*/

    pub fn acl_copy_entry(dest: acl_entry_t, src: acl_entry_t) -> c_int;
    pub fn acl_create_entry(acl: *mut acl_t, entry: *mut acl_entry_t) -> c_int;
    pub fn acl_delete_entry(acl: acl_t, entry: acl_entry_t) -> c_int;
    pub fn acl_get_entry(acl: acl_t, entry_id: c_int, entry: *mut acl_entry_t) -> c_int;

    /* Manipulate ACL entry permissions */

    pub fn acl_add_perm(permset: acl_permset_t, perm: acl_perm_t) -> c_int;
    pub fn acl_calc_mask(acl: *mut acl_t) -> c_int;
    pub fn acl_clear_perms(permset: acl_permset_t) -> c_int;
    pub fn acl_delete_perms(permset: acl_permset_t, perm: acl_perm_t) -> c_int;
    pub fn acl_get_permset(entry: acl_entry_t, permset: *mut acl_permset_t) -> c_int;
    pub fn acl_set_permset(entry: acl_entry_t, permset: acl_permset_t) -> c_int;

    /* Manipulate ACL entry tag type and qualifier */

    pub fn acl_get_qualifier(entry: acl_entry_t) -> *mut c_void;
    pub fn acl_get_tag_type(entry: acl_entry_t, tag_type: *const acl_tag_t) -> c_int;
    pub fn acl_set_qualifier(entry: acl_entry_t, tag_qualifier: *const c_void) -> c_int;
    pub fn acl_set_tag_type(entry: acl_entry_t, tag_type: acl_tag_t) -> c_int;

    /*=== Format translation ===*/

    pub fn acl_copy_ext(buf: *mut c_void, acl: acl_t, size: ssize_t) -> ssize_t;
    pub fn acl_copy_int(buf: *const c_void) -> acl_t;
    pub fn acl_from_text(buf: *const c_char) -> acl_t;
    pub fn acl_size(acl: acl_t) -> ssize_t;
    pub fn acl_to_text(acl: acl_t, len: *mut ssize_t) -> *mut c_char;

    /*=== Object manipulation ===*/

    pub fn acl_delete_def_file(path: *const c_char) -> c_int;
    pub fn acl_get_fd(fd: c_int) -> acl_t;
    pub fn acl_get_file(path: *const c_char, typ: acl_type_t) -> acl_t;
    pub fn acl_set_fd(fd: c_int, acl: acl_t) -> c_int;
    pub fn acl_set_file(path: *const c_char, typ: acl_type_t, acl: acl_t) -> c_int;
}
