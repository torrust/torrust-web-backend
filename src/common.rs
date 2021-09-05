use std::sync::Arc;
use crate::config::TorrustConfig;
use crate::database::Database;
use crate::auth::AuthorizationService;
use crate::tracker::TrackerService;

pub type Username = String;

pub type WebAppData = actix_web::web::Data<Arc<AppData>>;

pub struct AppData {
    pub cfg: Arc<TorrustConfig>,
    pub database: Arc<Database>,
    pub auth: Arc<AuthorizationService>,
    pub tracker: Arc<TrackerService>,
}

impl AppData {
    pub fn new(cfg: Arc<TorrustConfig>, database: Arc<Database>, auth: Arc<AuthorizationService>, tracker: Arc<TrackerService>) -> AppData {
        AppData {
            cfg,
            database,
            auth,
            tracker
        }
    }
}
