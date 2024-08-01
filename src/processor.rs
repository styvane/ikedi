//! This module defines the runner type
//!

use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::parser::DicomData;
use crate::DicomFileProcessor;

/// Write the data to the sink.
pub async fn write<S, P>(mut sink: S, mut recv: Receiver<DicomData<P>>) -> Result<()>
where
    S: AsyncWrite + Unpin,
    P: Serialize,
{
    while let Some(data) = recv.recv().await {
        let mut data = serde_json::to_string_pretty(&data)?;
        data.push('\n');
        sink.write_all(data.as_ref()).await?;
    }
    Ok(())
}

/// Read the data in the file as DicomData.
/// On success, it send the data to the sender.
pub async fn process_file(
    file: impl AsRef<Path>,
    sender: Sender<DicomData<PathBuf>>,
) -> Result<()> {
    match file.as_ref().open() {
        Ok(dicom_data) => {
            sender.send(dicom_data).await?;
        }
        Err(err) => {
            eprintln!("{err:?}");
            return Err(err);
        }
    }
    Ok(())
}
