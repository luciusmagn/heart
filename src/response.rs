use maud::Markup;
use serde::Serialize;
use warp::http::{Response, StatusCode};
use warp::reply;
use warp::reply::WithStatus;

pub fn hx_with(
    status: StatusCode,
    res: Markup,
) -> Response<String> {
    Response::builder()
        .header("Content", "text/html")
        .status(status)
        .body(res.into_string())
        // TODO: verify that this is the case
        .expect("BUG: impossible")
}

macro_rules! generate_hx_status_functions {
    ($($fname:ident, $status:ident);* $(;)?) => {
        $(
            pub fn $fname(res: Markup) -> Response<String> {
                hx_with(StatusCode::$status, res)
            }
        )*
    };
}

generate_hx_status_functions! {
    // funny aliases
    hx, OK;
    hx_error, IM_A_TEAPOT;

    // 1XX
    hx_continue, CONTINUE;
    hx_switching_protocols, SWITCHING_PROTOCOLS;
    hx_processing, PROCESSING;

    // 2XX
    hx_ok, OK;
    hx_created, CREATED;
    hx_accepted, ACCEPTED;
    hx_non_authoritative_information, NON_AUTHORITATIVE_INFORMATION;
    hx_no_content, NO_CONTENT;
    hx_reset_content, RESET_CONTENT;
    hx_partial_content, PARTIAL_CONTENT;
    hx_multi_status, MULTI_STATUS;
    hx_already_reported, ALREADY_REPORTED;
    hx_im_used, IM_USED;

    // 3XX
    hx_multiple_choices, MULTIPLE_CHOICES;
    hx_moved_permanently, MOVED_PERMANENTLY;
    hx_found, FOUND;
    hx_see_other, SEE_OTHER;
    hx_not_modified, NOT_MODIFIED;
    hx_use_proxy, USE_PROXY;
    hx_temporary_redirect, TEMPORARY_REDIRECT;
    hx_permanent_redirect, PERMANENT_REDIRECT;

    // 4XX
    hx_bad_request, BAD_REQUEST;
    hx_unauthorized, UNAUTHORIZED;
    hx_payment_required, PAYMENT_REQUIRED;
    hx_forbidden, FORBIDDEN;
    hx_not_found, NOT_FOUND;
    hx_method_not_allowed, METHOD_NOT_ALLOWED;
    hx_not_acceptable, NOT_ACCEPTABLE;
    hx_proxy_authentication_required, PROXY_AUTHENTICATION_REQUIRED;
    hx_request_timeout, REQUEST_TIMEOUT;
    hx_conflict, CONFLICT;
    hx_gone, GONE;
    hx_length_required, LENGTH_REQUIRED;
    hx_precondition_failed, PRECONDITION_FAILED;
    hx_payload_too_large, PAYLOAD_TOO_LARGE;
    hx_uri_too_long, URI_TOO_LONG;
    hx_unsupported_media_type, UNSUPPORTED_MEDIA_TYPE;
    hx_range_not_satisfiable, RANGE_NOT_SATISFIABLE;
    hx_expectation_failed, EXPECTATION_FAILED;
    hx_im_a_teapot, IM_A_TEAPOT;
    hx_misdirected_request, MISDIRECTED_REQUEST;
    hx_unprocessable_entity, UNPROCESSABLE_ENTITY;
    hx_locked, LOCKED;
    hx_failed_dependency, FAILED_DEPENDENCY;
    hx_upgrade_required, UPGRADE_REQUIRED;
    hx_precondition_required, PRECONDITION_REQUIRED;
    hx_too_many_requests, TOO_MANY_REQUESTS;
    hx_request_header_fields_too_large, REQUEST_HEADER_FIELDS_TOO_LARGE;
    hx_unavailable_for_legal_reasons, UNAVAILABLE_FOR_LEGAL_REASONS;

    // 5XX
    hx_internal_server_error, INTERNAL_SERVER_ERROR;
    hx_not_implemented, NOT_IMPLEMENTED;
    hx_bad_gateway, BAD_GATEWAY;
    hx_service_unavailable, SERVICE_UNAVAILABLE;
    hx_gateway_timeout, GATEWAY_TIMEOUT;
    hx_http_version_not_supported, HTTP_VERSION_NOT_SUPPORTED;
    hx_variant_also_negotiates, VARIANT_ALSO_NEGOTIATES;
    hx_insufficient_storage, INSUFFICIENT_STORAGE;
    hx_loop_detected, LOOP_DETECTED;
    hx_not_extended, NOT_EXTENDED;
    hx_network_authentication_required, NETWORK_AUTHENTICATION_REQUIRED
}

pub fn js_with<T: Serialize>(
    data: &T,
    status_code: &StatusCode,
) -> WithStatus<String> {
    let json =
        serde_json::to_string(data).expect("BUG: Impossible");
    reply::with_status(json, *status_code)
}

macro_rules! generate_js_status_functions {
    ($($fname:ident, $status:ident);* $(;)?) => {
        $(
            pub fn $fname<T: Serialize>(
                data: &T,
            ) -> WithStatus<String> {
                js_with(data, &StatusCode::$status)
            }
        )*
    };
}

generate_js_status_functions! {
    // funny aliases
    js, OK;
    js_error, IM_A_TEAPOT;

    // 1XX
    js_continue, CONTINUE;
    js_switching_protocols, SWITCHING_PROTOCOLS;
    js_processing, PROCESSING;

    // 2XX
    js_ok, OK;
    js_created, CREATED;
    js_accepted, ACCEPTED;
    js_non_authoritative_information, NON_AUTHORITATIVE_INFORMATION;
    js_no_content, NO_CONTENT;
    js_reset_content, RESET_CONTENT;
    js_partial_content, PARTIAL_CONTENT;
    js_multi_status, MULTI_STATUS;
    js_already_reported, ALREADY_REPORTED;
    js_im_used, IM_USED;

    // 3XX
    js_multiple_choices, MULTIPLE_CHOICES;
    js_moved_permanently, MOVED_PERMANENTLY;
    js_found, FOUND;
    js_see_other, SEE_OTHER;
    js_not_modified, NOT_MODIFIED;
    js_use_proxy, USE_PROXY;
    js_temporary_redirect, TEMPORARY_REDIRECT;
    js_permanent_redirect, PERMANENT_REDIRECT;

    // 4XX
    js_bad_request, BAD_REQUEST;
    js_unauthorized, UNAUTHORIZED;
    js_payment_required, PAYMENT_REQUIRED;
    js_forbidden, FORBIDDEN;
    js_not_found, NOT_FOUND;
    js_method_not_allowed, METHOD_NOT_ALLOWED;
    js_not_acceptable, NOT_ACCEPTABLE;
    js_proxy_authentication_required, PROXY_AUTHENTICATION_REQUIRED;
    js_request_timeout, REQUEST_TIMEOUT;
    js_conflict, CONFLICT;
    js_gone, GONE;
    js_length_required, LENGTH_REQUIRED;
    js_precondition_failed, PRECONDITION_FAILED;
    js_payload_too_large, PAYLOAD_TOO_LARGE;
    js_uri_too_long, URI_TOO_LONG;
    js_unsupported_media_type, UNSUPPORTED_MEDIA_TYPE;
    js_range_not_satisfiable, RANGE_NOT_SATISFIABLE;
    js_expectation_failed, EXPECTATION_FAILED;
    js_im_a_teapot, IM_A_TEAPOT;
    js_misdirected_request, MISDIRECTED_REQUEST;
    js_unprocessable_entity, UNPROCESSABLE_ENTITY;
    js_locked, LOCKED;
    js_failed_dependency, FAILED_DEPENDENCY;
    js_upgrade_required, UPGRADE_REQUIRED;
    js_precondition_required, PRECONDITION_REQUIRED;
    js_too_many_requests, TOO_MANY_REQUESTS;
    js_request_header_fields_too_large, REQUEST_HEADER_FIELDS_TOO_LARGE;
    js_unavailable_for_legal_reasons, UNAVAILABLE_FOR_LEGAL_REASONS;

    // 5XX
    js_internal_server_error, INTERNAL_SERVER_ERROR;
    js_not_implemented, NOT_IMPLEMENTED;
    js_bad_gateway, BAD_GATEWAY;
    js_service_unavailable, SERVICE_UNAVAILABLE;
    js_gateway_timeout, GATEWAY_TIMEOUT;
    js_http_version_not_supported, HTTP_VERSION_NOT_SUPPORTED;
    js_variant_also_negotiates, VARIANT_ALSO_NEGOTIATES;
    js_insufficient_storage, INSUFFICIENT_STORAGE;
    js_loop_detected, LOOP_DETECTED;
    js_not_extended, NOT_EXTENDED;
    js_network_authentication_required, NETWORK_AUTHENTICATION_REQUIRED
}
