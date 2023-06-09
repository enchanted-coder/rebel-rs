fn main() {

    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libopus-dev")
            .arg("build-essential")
            .arg("autoconf")
            .arg("automake")
            .arg("libtool")
            .arg("m4")
            .arg("ffmpeg")
            .arg("yt-dlp")
            
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }
    }
}