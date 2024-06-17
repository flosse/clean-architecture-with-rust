// NOTE:
// Originally we used `http` and `http-serde` here,
// but then we ran into problems because `warp` `v0.3`
// was always dependent on the outdated `http` `v0.2`
// and therefore other parts of the system could not be updated
// to the new `http` version `v1.x`.
// Now this crate is only dependent on `serde`.
// However, the mapping must now be done elsewhere.

use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

/// HTTP status code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusCode(pub NonZeroU16);

pub struct InvalidStatusCode;

impl InvalidStatusCode {
    const fn new() -> Self {
        Self {}
    }
}

impl StatusCode {
    pub fn from_u16(src: u16) -> Result<Self, InvalidStatusCode> {
        if !(100..1000).contains(&src) {
            return Err(InvalidStatusCode::new());
        }
        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    pub fn as_u16(&self) -> u16 {
        u16::from(self.0)
    }
}

macro_rules! status_codes {
    (
        $(
            ($num:expr, $konst:ident);
        )+
    ) =>
    {
        impl StatusCode {
            $(
                pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
            )+
        }
    }
}

status_codes! {
    (100, CONTINUE);
    (101, SWITCHING_PROTOCOLS);
    (102, PROCESSING);
    (200, OK);
    (201, CREATED);
    (202, ACCEPTED);
    (203, NON_AUTHORITATIVE_INFORMATION);
    (204, NO_CONTENT);
    (205, RESET_CONTENT);
    (206, PARTIAL_CONTENT);
    (207, MULTI_STATUS);
    (208, ALREADY_REPORTED);
    (226, IM_USED);
    (300, MULTIPLE_CHOICES);
    (301, MOVED_PERMANENTLY);
    (302, FOUND);
    (303, SEE_OTHER);
    (304, NOT_MODIFIED);
    (305, USE_PROXY);
    (307, TEMPORARY_REDIRECT);
    (308, PERMANENT_REDIRECT);
    (400, BAD_REQUEST);
    (401, UNAUTHORIZED);
    (402, PAYMENT_REQUIRED);
    (403, FORBIDDEN);
    (404, NOT_FOUND);
    (405, METHOD_NOT_ALLOWED);
    (406, NOT_ACCEPTABLE);
    (407, PROXY_AUTHENTICATION_REQUIRED);
    (408, REQUEST_TIMEOUT);
    (409, CONFLICT);
    (410, GONE);
    (411, LENGTH_REQUIRED);
    (412, PRECONDITION_FAILED);
    (413, PAYLOAD_TOO_LARGE);
    (414, URI_TOO_LONG);
    (415, UNSUPPORTED_MEDIA_TYPE);
    (416, RANGE_NOT_SATISFIABLE);
    (417, EXPECTATION_FAILED);
    (418, IM_A_TEAPOT);
    (421, MISDIRECTED_REQUEST);
    (422, UNPROCESSABLE_ENTITY);
    (423, LOCKED);
    (424, FAILED_DEPENDENCY);
    (426, UPGRADE_REQUIRED);
    (428, PRECONDITION_REQUIRED);
    (429, TOO_MANY_REQUESTS);
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE);
    (451, UNAVAILABLE_FOR_LEGAL_REASONS);
    (500, INTERNAL_SERVER_ERROR);
    (501, NOT_IMPLEMENTED);
    (502, BAD_GATEWAY);
    (503, SERVICE_UNAVAILABLE);
    (504, GATEWAY_TIMEOUT);
    (505, HTTP_VERSION_NOT_SUPPORTED);
    (506, VARIANT_ALSO_NEGOTIATES);
    (507, INSUFFICIENT_STORAGE);
    (508, LOOP_DETECTED);
    (510, NOT_EXTENDED);
    (511, NETWORK_AUTHENTICATION_REQUIRED);
}
