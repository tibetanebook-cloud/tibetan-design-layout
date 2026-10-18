//! HTTPS release metadata only: no installer download, execution or document access.
use designcraft_ui_egui::DesignApp;
use std::sync::mpsc::{Receiver, TryRecvError};
#[derive(Clone, Debug, PartialEq, Eq)]
struct Release {
    version: String,
    url: String,
}
type Pending = Receiver<Result<Option<Release>, String>>;
use semver::Version;
use serde_json::Value;
use std::time::Duration;
const API: &str = "https://api.github.com/repos/tibetanebook-cloud/tibetan-design-layout/releases/latest";
const REPO: &str = "https://github.com/tibetanebook-cloud/tibetan-design-layout";
fn platform_suffix() -> Option<&'static str> {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some("windows-x64.msi")
    } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
        Some("windows-arm64.msi")
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Some("macos-aarch64.dmg")
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        Some("macos-x86_64.dmg")
    } else {
        None
    }
}
fn parse_release(body: &str, current: &str, suffix: &str) -> Result<Option<Release>, String> {
    let json: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if json.get("draft").and_then(Value::as_bool) != Some(false) || json.get("prerelease").and_then(Value::as_bool) != Some(false) {
        return Ok(None);
    }
    let tag = json.get("tag_name").and_then(Value::as_str).filter(|s| s.len() <= 80).ok_or("Invalid release version")?;
    let version = Version::parse(tag.strip_prefix('v').unwrap_or(tag)).map_err(|e| e.to_string())?;
    let installed = Version::parse(current).map_err(|e| e.to_string())?;
    if !version.pre.is_empty() || version <= installed {
        return Ok(None);
    }
    let url = format!("{REPO}/releases/tag/{tag}");
    if json.get("html_url").and_then(Value::as_str) != Some(url.as_str()) {
        return Err("Unexpected release page URL".into());
    }
    let name = format!("tibetan-design-layout-{version}-{suffix}");
    let asset_url = format!("{REPO}/releases/download/{tag}/{name}");
    let compatible = json.get("assets").and_then(Value::as_array).is_some_and(|assets| {
        assets.iter().any(|a| {
            a.get("name").and_then(Value::as_str) == Some(name.as_str())
                && a.get("browser_download_url").and_then(Value::as_str) == Some(asset_url.as_str())
        })
    });
    Ok(compatible.then(|| Release { version: version.to_string(), url }))
}
fn latest() -> Result<Option<Release>, String> {
    let Some(suffix) = platform_suffix() else { return Ok(None) };
    let config = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(12))).build();
    let agent: ureq::Agent = config.into();
    let mut response = agent
        .get(API)
        .header("User-Agent", "TibetanDesign-update-check")
        .header("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| e.to_string())?;
    let body = response.body_mut().with_config().limit(64 * 1024).read_to_string().map_err(|e| e.to_string())?;
    parse_release(&body, env!("CARGO_PKG_VERSION"), suffix)
}
fn check(ctx: &egui::Context) -> Result<Pending, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::Builder::new()
        .name("tibetan-update-check".into())
        .spawn(move || {
            let result = latest();
            let _ = tx.send(result);
            ctx.request_repaint();
        })
        .map_err(|e| e.to_string())?;
    Ok(rx)
}

#[derive(Default)]
pub struct State {
    pending: Option<Pending>,
    manual: bool,
    offer: Option<Release>,
    message: Option<String>,
    started: Option<std::time::Instant>,
}
impl State {
    pub fn tick(&mut self, app: &mut DesignApp, ctx: &egui::Context) {
        let age = self.started.get_or_insert_with(std::time::Instant::now).elapsed();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let requested = std::mem::take(&mut app.ui.update_requested);
        let due = app.ui.automatic_updates
            && age >= Duration::from_secs(15)
            && (now.saturating_sub(app.ui.last_update_check) >= 86400 || now < app.ui.last_update_check);
        if requested || due {
            self.manual |= requested;
            if self.pending.is_none() {
                app.ui.last_update_check = now;
                match check(ctx) {
                    Ok(rx) => self.pending = Some(rx),
                    Err(e) => {
                        if self.manual {
                            self.message = Some(e);
                        }
                        self.manual = false;
                    }
                }
            }
        }
        let result = self.pending.as_ref().and_then(|rx| match rx.try_recv() {
            Ok(r) => Some(r),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err("Update service stopped. Please try again.".into())),
        });
        if let Some(result) = result {
            self.pending = None;
            match result {
                Ok(Some(release)) if self.manual || release.version != app.ui.skipped_update => self.offer = Some(release),
                Ok(_) if self.manual => {
                    self.message = Some(
                        if app.ui.language == "bo" {
                            "ཐོན་རིམ་གསར་པ་མི་འདུག"
                        } else {
                            "No newer release is available."
                        }
                        .into(),
                    )
                }
                Err(e) if self.manual => self.message = Some(format!("Could not check for updates: {e}")),
                _ => {}
            }
            self.manual = false;
        }
        ctx.request_repaint_after(if self.pending.is_some() || age < Duration::from_secs(15) {
            Duration::from_secs(1)
        } else {
            Duration::from_secs(60)
        });
    }
    pub fn show(&mut self, app: &mut DesignApp, ctx: &egui::Context) {
        let bo = app.ui.language == "bo";
        if let Some(release) = self.offer.clone() {
            let mut open = true;
            let mut close = false;
            egui::Window::new(if bo { "ཐོན་རིམ་གསར་པ་འདུག" } else { "New version available" }).open(&mut open).collapsible(false).show(ctx, |ui| {
                ui.label(format!("Tibetan Design Layout {}", release.version));
                ui.horizontal(|ui| {
                    if ui.button(if bo { "ཕབ་ལེན་ཤོག་ངོས་ཁ་འབྱེད།" } else { "Open download page" }).clicked()
                    {
                        ctx.open_url(egui::OpenUrl::new_tab(&release.url));
                        close = true;
                    }
                    if ui.button(if bo { "རྗེས་སུ།" } else { "Later" }).clicked() {
                        close = true;
                    }
                    if ui.button(if bo { "ཐོན་རིམ་འདི་སྐྱུར།" } else { "Skip this version" }).clicked() {
                        app.ui.skipped_update = release.version.clone();
                        close = true;
                    }
                });
            });
            if close || !open {
                self.offer = None;
            }
        }
        if let Some(message) = self.message.clone() {
            let mut open = true;
            egui::Window::new(if bo { "གསར་སྒྱུར།" } else { "Updates" }).open(&mut open).show(ctx, |ui| {
                ui.label(message);
            });
            if !open {
                self.message = None;
            }
        }
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;
    fn ready(result: Result<Option<Release>, String>, manual: bool) -> State {
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(result).unwrap();
        State { pending: Some(rx), manual, ..Default::default() }
    }
    #[test]
    fn automatic_failures_are_quiet_but_manual_failures_are_shown() {
        let mut app = DesignApp::new(designcraft_engine::Session::new(), Default::default());
        let ctx = egui::Context::default();
        let mut state = ready(Err("offline".into()), false);
        state.tick(&mut app, &ctx);
        assert!(state.message.is_none());
        let mut state = ready(Err("offline".into()), true);
        state.tick(&mut app, &ctx);
        assert!(state.message.as_deref().unwrap().contains("offline"));
    }
    #[test]
    fn manual_check_reveals_a_skipped_version_and_preserves_preferences() {
        let mut app = DesignApp::new(designcraft_engine::Session::new(), Default::default());
        app.ui.automatic_updates = false;
        app.ui.skipped_update = "9.0.0".into();
        let ctx = egui::Context::default();
        let release = Release { version: "9.0.0".into(), url: REPO.into() };
        let mut state = ready(Ok(Some(release.clone())), false);
        state.tick(&mut app, &ctx);
        assert!(state.offer.is_none());
        let mut state = ready(Ok(Some(release.clone())), true);
        state.tick(&mut app, &ctx);
        assert_eq!(state.offer, Some(release));
        let bytes = serde_json::to_vec(&app.ui).unwrap();
        let saved = designcraft_ui_egui::UiState::from_preferences_json(&bytes).unwrap();
        assert!(!saved.automatic_updates);
        assert_eq!(saved.skipped_update, "9.0.0");
        assert!(!saved.update_requested);
        assert!(app.session.documents().is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn release(tag: &str, suffix: &str) -> Value {
        let version = tag.strip_prefix('v').unwrap_or(tag);
        let name = format!("tibetan-design-layout-{version}-{suffix}");
        json!({"draft":false,"prerelease":false,"tag_name":tag,"html_url":format!("{REPO}/releases/tag/{tag}"),
            "assets":[{"name":name,"browser_download_url":format!("{REPO}/releases/download/{tag}/{name}")}]})
    }
    #[test]
    fn update_versions_compare_numerically_and_require_platform_assets() {
        let body = release("v0.10.0", "windows-x64.msi").to_string();
        assert_eq!(parse_release(&body, "0.9.9", "windows-x64.msi").unwrap().unwrap().version, "0.10.0");
        assert!(parse_release(&body, "0.10.0", "windows-x64.msi").unwrap().is_none());
        assert!(parse_release(&body, "1.0.0", "windows-x64.msi").unwrap().is_none());
        assert!(parse_release(&body, "0.9.0", "macos-aarch64.dmg").unwrap().is_none());
    }
    #[test]
    fn drafts_prereleases_invalid_json_and_other_hosts_are_rejected() {
        let mut body = release("v1.0.0", "macos-x86_64.dmg");
        body["draft"] = json!(true);
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").unwrap().is_none());
        body["draft"] = json!(false);
        body["prerelease"] = json!(true);
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").unwrap().is_none());
        body["prerelease"] = json!(false);
        body["html_url"] = json!("https://example.com/update");
        assert!(parse_release(&body.to_string(), "0.8.3", "macos-x86_64.dmg").is_err());
        assert!(parse_release("not json", "0.8.3", "windows-x64.msi").is_err());
        assert!(parse_release(&release("v1.0.0-beta.1", "windows-x64.msi").to_string(), "0.8.3", "windows-x64.msi").unwrap().is_none());
    }
}
