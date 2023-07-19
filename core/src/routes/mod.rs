use std::{path::PathBuf, sync::Arc};

use rspc::{Config, ErrorCode, Router};

use crate::Shared;

pub mod db;

pub fn init_router() -> Arc<Router<Shared>> {
    let router = Router::<Shared>::new()
        .config(
            Config::new()
                .set_ts_bindings_header("/* eslint-disable */")
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.d.ts"),
                ),
        )
        .merge("db.", db::mount()) <-- example
        .query("version", |t| {
            t(|_, _: ()| async move { env!("CARGO_PKG_VERSION") })
        })
        .query("openInDefault", |t| {
            t(|_, data: String| {
                open::that(data).map_err(|err| -> rspc::Error {
                    rspc::Error::new(ErrorCode::InternalServerError, err.to_string())
                })
            })
        });

    router.build().arced()
}
