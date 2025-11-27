#[cfg(feature = "async")]
use super::AsyncUpdater;
use super::Updater;

pub struct DummyUpdater;
#[cfg(feature = "async")]
unsafe impl Send for DummyUpdater {}
#[cfg(feature = "async")]
unsafe impl Sync for DummyUpdater {}

impl DummyUpdater {
    pub fn new() -> Self {
        Self
    }
}

impl Updater for DummyUpdater {
    fn record_scan_progress(
        &mut self,
        _start: bitcoin::absolute::Height,
        _current: bitcoin::absolute::Height,
        _end: bitcoin::absolute::Height,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn record_block_outputs(
        &mut self,
        _height: bitcoin::absolute::Height,
        _blkhash: bitcoin::BlockHash,
        _found_outputs: std::collections::HashMap<bitcoin::OutPoint, crate::OwnedOutput>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn record_block_inputs(
        &mut self,
        _blkheight: bitcoin::absolute::Height,
        _blkhash: bitcoin::BlockHash,
        _found_inputs: std::collections::HashSet<bitcoin::OutPoint>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn save_to_persistent_storage(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "async")]
impl AsyncUpdater for DummyUpdater {
    async fn record_scan_progress(
        &mut self,
        _start: bitcoin::absolute::Height,
        _current: bitcoin::absolute::Height,
        _end: bitcoin::absolute::Height,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn record_block_outputs(
        &mut self,
        _height: bitcoin::absolute::Height,
        _blkhash: bitcoin::BlockHash,
        _found_outputs: std::collections::HashMap<bitcoin::OutPoint, crate::OwnedOutput>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn record_block_inputs(
        &mut self,
        _blkheight: bitcoin::absolute::Height,
        _blkhash: bitcoin::BlockHash,
        _found_inputs: std::collections::HashSet<bitcoin::OutPoint>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn save_to_persistent_storage(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
