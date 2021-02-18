
use serde::Deserialize;
use serde::Serialize;
use serde_qs as qs;
use uuid::Uuid;
use yew::callback::Callback;
use yew::format::Text;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, StatusCode};
use yew::services::storage::{Area, StorageService};
use yew::ComponentLink;

use crate::app::app::{Model as App, Msg as AppMsg};
use crate::app::pages::login::AuthResponse;
use crate::app::services::routes::Route;

pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub struct JWT {
    sub: Uuid,
    exp: i32,
    ...
}

pub struct HttpClient {
    pub link: Option<ComponentLink<App>>,
    storage_service: StorageService,
    active_requests: Vec<FetchTask>,
    raw_jwt: Option<String>,
    jwt: Option<JWT>,
    pub token: Option<String>,
}

impl Default for HttpClient {
    fn default() -> Self {
        let storage_service = StorageService::new(Area::Local);
        let raw_jwt = match storage_service.restore::<Text>("jwt") {
            Ok(raw_jwt) => Some(raw_jwt),
            Err(_) => None,
        };
        let token = match storage_service.restore::<Text>("token") {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        let jwt = match raw_jwt {
            Some(ref raw_jwt) => {
                Some(decode_jwt(raw_jwt).expect("Could not decode jwt in local storage"))
            }
            None => None,
        };

        HttpClient {
            link: None,
            storage_service,
            active_requests: vec![],
            raw_jwt,
            jwt,
            token,
        }
    }
}

impl HttpClient {
    pub fn req<R>(
        &mut self,
        route: &Route<R>,
        dyn_path: &str,
        data: impl Into<Text>,
        params: Option<impl Serialize>,
        callback: Callback<Response<R>>,
    ) -> Result<(), String>
    where
        R: 'static + From<Text>,
    {
        let uri = format!("{}/{}{}", route.api, route.path, dyn_path);
        let uri = match params {
            Some(params) => format!(
                "{}?{}",
                uri,
                qs::to_string(&params).unwrap_or("".to_owned())
            ),
            None => uri,
        };
        let mut request = match route.method {
            Method::Get => Request::get(uri),
            Method::Post => Request::post(uri),
            Method::Patch => Request::patch(uri),
            Method::Delete => Request::delete(uri),
        };

        let request = match self.raw_jwt {
            Some(ref jwt) => request.header("Authorization", format!("Bearer {}", jwt)),
            None => &mut request,
        }
        .body(data);

        let self_callback = match self.link {
            Some(ref mut link) => link.send_back(move |response: Response<R>| {
                let (meta, body) = response.into_parts();

                if meta.status == StatusCode::UNAUTHORIZED {
                    AppMsg::LoggedOut
                } else {
                    callback.emit(Response::from_parts(meta, body));
                    AppMsg::Noop
                }
            }),
            None => return Err("No link for request".to_owned()),
        };

        self.active_requests.push(
            FetchService::new().fetch(request.expect("Could not build request"), self_callback),
        );

        Ok(())
    }

    pub fn reset_credentials(&mut self) {
        ...
    }

    pub fn set_credentials(&mut self, payload: AuthResponse) -> Result<(), String> {
        ...
    }

    pub fn is_logged_in(&self) -> bool {
        self.token.is_some()
    }
}

fn decode_jwt(jwt: &str) -> Result<JWT, String> {
    ...
}