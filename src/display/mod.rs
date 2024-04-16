pub mod icons;

use crate::fetch::Fetch;
use std::fmt;

impl<'cfg> fmt::Display for Fetch<'cfg> {
    fn fmt(&self, stream: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_prefix_len = self
            .sources
            .iter()
            .map(|source| source.text_prefix().len())
            .max()
            .unwrap_or_default();

        writeln!(stream)?; // The upper blank line.
        for source in &self.sources {
            let icon = match self.config.icons {
                true => format!("{} ", source.icon()),
                false => source.text_prefix(),
            };
            writeln!(stream, "\t{:>max_prefix_len$} │ {}", icon, source.info())?;
        }

        Ok(())
    }
}
