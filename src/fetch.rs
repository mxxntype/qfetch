use crate::sources::colors::Colors;
use crate::sources::disk::Disk;
use crate::sources::host::Host;
use crate::sources::os::Os;
use crate::sources::ram::Ram;
use crate::sources::user::User;
use crate::{config::Config, sources::FetchSource};
use sysinfo::{RefreshKind, System, SystemExt};

/// The fetch itself, represented as a struct.
///
/// This struct holds all the information it needs to display, as well
/// as the [`Config`] options that may be passed via CLI arguments.
pub struct Fetch<'cfg> {
    pub(crate) sources: Vec<Box<dyn FetchSource>>,
    pub(crate) config: &'cfg Config,
}

impl<'cfg> Fetch<'cfg> {
    /// Instantiate a new [`Fetch`].
    ///
    /// An instance created this way will respect options in the provided [`Config`].
    pub fn new_with_config(config: &'cfg Config) -> Self {
        let system = System::new_with_specifics(RefreshKind::new().with_memory().with_disks_list());
        let user = User::get();
        let host = Host::get(&system);
        let os = Os::get(&system);
        let ram = Ram::new(&system);
        let disk = Disk::get(&system);
        let colors = Colors::get();

        Self {
            config,
            sources: vec![
                Box::new(user),
                Box::new(host),
                Box::new(os),
                Box::new(ram),
                Box::new(disk),
                Box::new(colors),
            ],
        }
    }
}
