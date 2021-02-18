use lazy_static::lazy_static;
use std::marker::PhantomData;

use crate::app::services::http_client::Method;

pub enum Api {
    Licenses,
    Auth,
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Licenses => write!(f, "{}", env!("LICENSES_API")),
            Self::Auth => write!(f, "{}", env!("AUTH_API")),
        }
    }
}

pub struct Route<R> {
    pub api: Api,
    pub path: &'static str,
    pub method: Method,
    pub auth: bool,
    pub _marker: PhantomData<R>,
}

use crate::app::models::coupon::Coupon;
use crate::app::models::stripe_coupon::StripeCoupon;
use crate::app::pages::login::LoginResponse;
use yew::format::Json;

pub struct Routes {
    pub login: Route<Json<Result<LoginResponse, failure::Error>>>,
    pub fetch_coupons: Route<Json<Result<Vec<Coupon>, failure::Error>>>,
    pub update_coupon: Route<Json<Result<Coupon, failure::Error>>>,
    pub create_coupon: Route<Json<Result<Coupon, failure::Error>>>,
    pub delete_coupon: Route<()>,
}

lazy_static! {
    pub static ref ROUTES: Routes = Routes {
        login: Route {
            api: Api::Auth,
            path: "/users/login",
            method: Method::Post,
            auth: false,
            _marker: PhantomData,
        },
        fetch_coupons: Route {
            api: Api::Licenses,
            path: "/coupons",
            method: Method::Get,
            auth: true,
            _marker: PhantomData,
        },
        update_coupon: Route {
            api: Api::Licenses,
            path: "/coupons",
            method: Method::Patch,
            auth: true,
            _marker: PhantomData,
        },
        create_coupon: Route {
            api: Api::Licenses,
            path: "/coupons",
            method: Method::Post,
            auth: true,
            _marker: PhantomData,
        },
        delete_coupon: Route {
            api: Api::Licenses,
            path: "/coupons",
            method: Method::Delete,
            auth: true,
            _marker: PhantomData,
        }
    };
}