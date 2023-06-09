/* 
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
            //.arg("yt-dlp")
            
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }
    }
}
*/
// dep-installer-hack/build.rs

fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libopus-dev") // the apt package that a dependency of my project needs to compile
            // can add more here
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }
    }
}