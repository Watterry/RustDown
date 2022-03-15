use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;


mod publish;
use crate::publish::bookdown;

fn main() -> std::io::Result<()> {
    println!("current folder");

    let folder = env::current_dir()?;
    println!("The current directory is {}", folder.display());


    //TODO: should just read .md files
    let bookfolder = "./testbook";
    let paths = fs::read_dir(bookfolder).unwrap();


    let mut file = File::create("output.md").unwrap();

    //write index.md file first
    let mut owned_string: String = bookfolder.to_owned();
    let borrowed_string: &str = "/index.md";
    owned_string.push_str(borrowed_string);

    let start_content = fs::read_to_string(owned_string)
                       .expect("Something went wrong reading the file");
    println!("With text:\n{}", start_content);
    file.write_all(start_content.as_bytes()).unwrap();

    // read file one by one and write into a markdown file together
    for entry in paths {
        //println!("Name: {}", &entry.unwrap().path().display());
        //print_type_of(&path);

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            //visit_dirs(&path, cb)?;
            println!("directory, here should be some error!");
        } else {
            let filename = path.file_name().unwrap().to_os_string().into_string().unwrap();
            println!( "filename:{}\n", filename );

            let banana: &str = "index.md";
            if !filename.eq(banana)
            {
                let contents = fs::read_to_string(path)
                               .expect("Something went wrong reading the file");
                println!("With text:\n{}", contents);

                file.write_all(contents.as_bytes()).unwrap();
            }
        }
    }

    drop(file);

    bookdown::publish("output.md");

    Ok(())
}
