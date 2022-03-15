pub mod bookdown {

    use std::process::Command;

    pub fn publish(filename: &str) {
        //parse YAML of markdown
        

        pdf(filename);
        html(filename);

    }

    fn pdf(filename: &str) {
        //pandoc
        let output = Command::new("pandoc")
                             .arg(filename)
                             .arg("--pdf-engine=xelatex")
                             .arg("-o")
                             .arg("example4.pdf")
                             .output()
                             .expect("failed to execute process");

        if !output.status.success() {
            println!("Command executed with failing error code");
        }
    }

    fn tex(filename: &str) {
        //pandoc
        let output = Command::new("pandoc")
                             .arg("-s")
                             .arg(filename)
                             .arg("-o")
                             .arg("example4.tex")
                             .output()
                             .expect("failed to execute process");

        if !output.status.success() {
            println!("Command executed with failing error code");
        }
    }

    fn html(filename: &str) {
        //pandoc
        let output = Command::new("pandoc")
                             .arg("-s")
                             .arg(filename)
                             .arg("-o")
                             .arg("example4.html")
                             .output()
                             .expect("failed to execute process");

        if !output.status.success() {
            println!("Command executed with failing error code");
        }
    }
}