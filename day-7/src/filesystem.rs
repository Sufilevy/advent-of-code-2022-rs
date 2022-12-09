use std::collections::HashMap;

type Directory = HashMap<String, u64>;

#[derive(Debug)]
enum Entry {
    Directory { name: String },
    File { name: String, size: u64 },
}

impl Entry {
    pub fn from(input: &str) -> Self {
        if input.starts_with("dir") {
            Self::from_directory(input)
        } else {
            Self::from_file(input)
        }
    }

    fn from_directory(input: &str) -> Self {
        let ["dir", name] = input.split_whitespace().collect::<Vec<_>>()[..] else { panic!("No.") };
        Self::Directory {
            name: name.to_string(),
        }
    }

    fn from_file(input: &str) -> Self {
        let [size, name] = input.split_whitespace().collect::<Vec<_>>()[..] else { panic!("Why?") };
        let size = size.parse::<u64>().unwrap();
        Self::File {
            name: name.to_string(),
            size,
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<Entry>),
}

impl Command {
    pub fn from(input: &[&str]) -> Self {
        match input[0].split_whitespace().collect::<Vec<_>>()[..] {
            ["cd", directory] => Self::Cd(directory.to_string()),
            ["ls"] => Self::Ls(
                input[1..]
                    .iter()
                    .map(|line| Entry::from(line))
                    .collect::<Vec<_>>(),
            ),
            _ => panic!("..."),
        }
    }
}

pub struct Terminal {
    directories: HashMap<String, Directory>,
    current_path: String,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            directories: HashMap::new(),
            current_path: String::new(),
        }
    }

    pub fn interpret(&mut self, input: &str) -> &mut Self {
        input.split("$ ").skip(1).for_each(|lines| {
            let command = Command::from(&lines.lines().collect::<Vec<_>>());
            self.execute(command);
        });

        self
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::Cd(directory) => self.cd(directory),
            Command::Ls(entries) => self.ls(entries),
        }
    }

    fn cd(&mut self, directory_name: String) {
        if directory_name == ".." {
            let index = self.current_path.rfind('/').unwrap();
            self.current_path = self.current_path.split_at(index).0.to_string();
        } else if directory_name == "/" {
            self.current_path = directory_name;
            self.add_directory(self.current_path.clone());
        } else {
            self.add_directory(directory_name.clone());
            self.current_path = if self.current_path == "/" {
                format!("/{}", directory_name)
            } else {
                format!("{}/{}", self.current_path, directory_name)
            };
        }
    }

    fn ls(&mut self, entries: Vec<Entry>) {
        for entry in entries {
            match entry {
                Entry::Directory { name } => self.add_directory(name),
                Entry::File { name, size } => self.add_file(name, size),
            }
        }
    }

    fn add_directory(&mut self, directory_name: String) {
        let directory_path = if directory_name == "/" {
            directory_name
        } else if self.current_path == "/" {
            format!("/{}", directory_name)
        } else {
            format!("{}/{}", self.current_path, directory_name)
        };
        self.directories
            .entry(directory_path)
            .or_insert_with(Directory::new);
    }

    fn add_file(&mut self, name: String, size: u64) {
        self.directories
            .get_mut(&self.current_path)
            .unwrap()
            .insert(name, size);
    }

    pub fn sum_small_directories(&self) -> u64 {
        self.directories
            .iter()
            .filter_map(|directory| {
                Self::sum_small_directory(self.filter_sub_directories(directory.0))
            })
            .sum()
    }

    fn filter_sub_directories(&self, directory_name: &String) -> Vec<(&String, &Directory)> {
        self.directories
            .iter()
            .filter(|d| d.0.starts_with(directory_name))
            .collect()
    }

    fn sum_small_directory(sub_directories: Vec<(&String, &Directory)>) -> Option<u64> {
        let sum = Self::sum_sub_directories(sub_directories);
        if sum <= 100_000 {
            Some(sum)
        } else {
            None
        }
    }

    fn sum_sub_directories(sub_directories: Vec<(&String, &Directory)>) -> u64 {
        sub_directories
            .iter()
            .map(|directory| directory.1.clone().into_values().sum::<u64>())
            .sum()
    }

    pub fn find_sufficient_directory(&self) -> u64 {
        let mut sums = self.sum_directories();
        sums.sort();

        let root_size = sums.last().unwrap();
        let required_space = 30_000_000 - (70_000_000 - root_size);

        sums.into_iter()
            .find(|directory_size| *directory_size > required_space)
            .unwrap()
    }

    fn sum_directories(&self) -> Vec<u64> {
        self.directories
            .iter()
            .map(|directory| Self::sum_sub_directories(self.filter_sub_directories(directory.0)))
            .collect()
    }
}
