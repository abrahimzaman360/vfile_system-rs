#[allow(dead_code)]

mod file_system {
    #[derive(Debug)]
    struct File {
        id: u32,
        name: String,
        path: String,
        f_type: String,
        size: f32,
    }

    #[derive(Debug)]
    struct Folder {
        id: u32,
        name: String,
        path: String,
        size: f32,
        color: (f32, f32, f32),
    }

    #[derive(Debug)]
    pub struct FileSystem {
        files: Vec<File>,
        folders: Vec<Folder>,
    }

    impl FileSystem {
        pub fn init() -> Self {
            Self {
                files: vec![],
                folders: vec![],
            }
        }

        pub fn new_file(&mut self) {
            let new_file = File {
                id: 1,
                name: "BigBang.txt".to_string(),
                f_type: "txt".to_string(),
                size: 12.2,
                path: "/parent".to_string(),
            };

            self.files.push(new_file);
        }

        pub fn print_files(&self) {
            for each_file in self.files.iter() {
                println!(
                    "\nId: {:?},\t Name: {:?},\t Type: {:?},\t Path: {:?},\t Size: {:?}",
                    each_file.id, each_file.name, each_file.f_type, each_file.path, each_file.size
                );
            }
        }

        pub fn print_folders(&self) {
            if self.folders.len() > 0 {
                for each_folder in self.folders.iter() {
                    println!(
                        "File:
                        ID: {:?}, Name: {:?}, Path: {:?}, Size: {:?}, Folder color: {:?}",
                        each_folder.id,
                        each_folder.name,
                        each_folder.path,
                        each_folder.size,
                        each_folder.color
                    );
                }
            } else {
                print!("~root > ")
            }
        }
    }
}

fn main() {
    let mut new_filesystem = file_system::FileSystem::init();
    new_filesystem.new_file();
    new_filesystem.print_folders();
    new_filesystem.print_files();
}
