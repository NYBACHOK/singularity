/// Simple method which check is nvidia drivers present in system. This ignores intel gpu and igpu, but for now should be sufficient
/// to give rocm support for users with amd
pub async fn is_nvidia() -> bool {
    match tokio::process::Command::new("nvidia-smi").spawn() {
        Ok(child) => child,
        Err(e) => {
            tracing::warn!("Failed to check gpu. Reason: {e}");

            return false;
        }
    }
    .wait()
    .await
    .map(|this| this.success())
    .unwrap_or_default()
}
