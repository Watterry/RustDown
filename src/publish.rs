pub mod bookdown {

    use std::process::Command;

    use yaml_front_matter::{Document, YamlFrontMatter};
    pub fn publish(filename: &str) {
        //parse YAML of markdown as parameters to pandoc
        const SIMPLE_MARKDOWN_YFM: &str = r#"
        ---
        title: 'Parsing a Markdown file metadata into a struct'
        description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
        tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
        similar_posts:
          - 'Rendering markdown'
          - 'Using Rust to render markdown'
        date: '2021-09-13T03:48:00'
        favorite_numbers:
            - 3.14
            - 1970
            - 12345
        ---
        
        
        
        > This tutorial walks you through the practice of parsing markdown files for metadata
        "#;

        struct Metadata {
            title: String,
            description: String,
            tags: Vec<String>,
            similar_posts: Vec<String>,
            date: String,
            favorite_numbers: Vec<f64>,
        }
        
        let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&SIMPLE_MARKDOWN_YFM).unwrap();
        
        let Metadata {
            title,
            description,
            tags,
            similar_posts,
            date,
            favorite_numbers,
        } = document.metadata;        


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