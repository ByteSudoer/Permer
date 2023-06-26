#![allow(unused_must_use)]
use crate::utils::*;
use cli_table::{print_stdout, Table, WithTitle};
use std::env;
use std::fmt;
use std::fs;
use std::os::unix::fs::MetadataExt;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Table, Debug)]
pub struct Permessions {
    #[table(title = "", Justify = "Justify::Right")]
    element: String,
    #[table(title = "Read")]
    read_access: bool,
    #[table(title = "write")]
    write_access: bool,
    #[table(title = "execute")]
    execute_access: bool,
}

impl Permessions {
    pub fn new(name: &str, user: i32) -> Self {
        let permessions = get_permessions(name, user);
        let element = match user {
            1 => "User".to_owned(),
            2 => "Group".to_owned(),
            3 => "Others".to_owned(),
            _ => unreachable!("error element not available"),
        };
        Self {
            element,
            read_access: permessions[0],
            write_access: permessions[1],
            execute_access: permessions[2],
        }
    }
}
#[derive(Debug)]
pub struct File {
    name: String,
    path: String,
    owner: String,
    group: String,
    inode: u64,
    accesstime: String,
    modificationtime: String,
    permessions: [Permessions; 3],
    size: String,
}

impl File {
    pub fn new(input: &str) -> Self {
        Self {
            name: input.to_string(),
            path: get_full_path(input),
            owner: user_byid(get_user_and_group(input).0),
            group: group_byid(get_user_and_group(input).1),
            inode: get_inode_number(input),
            accesstime: unix_timestamp_to_datetime(get_time_info(input).0),
            modificationtime: unix_timestamp_to_datetime(get_time_info(input).1),
            permessions: [
                Permessions::new(input, 1),
                Permessions::new(input, 2),
                Permessions::new(input, 3),
            ],
            size: convert_byte_human(get_size(input)),
        }
    }
}

// This function returns the absolute path of the file passed as argument
pub fn get_full_path(name: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    current_dir.join(name).display().to_string()
}

// function to return UID (user id) ang GID(group id) from file name
pub fn get_user_and_group(name: &str) -> (u32, u32) {
    let metadata = fs::metadata(name).unwrap();
    let user_id = metadata.uid();
    let group_id = metadata.gid();

    (user_id, group_id)
}

// This function takes the user ID as a paramter and returns the user name
pub fn user_byid(id: u32) -> String {
    get_user_by_uid(id)
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .to_string()
}

// This function takes the group ID as a paramter and returns the group name
pub fn group_byid(id: u32) -> String {
    get_group_by_gid(id)
        .unwrap()
        .name()
        .to_str()
        .unwrap()
        .to_string()
}

// This function takes the file name as an argument and returns the last access time and last
// modification time
pub fn get_time_info(name: &str) -> (i64, i64) {
    let metadata = fs::metadata(name).unwrap();
    (metadata.atime(), metadata.mtime())
}
pub fn get_inode_number(name: &str) -> u64 {
    let metadata = fs::metadata(name).unwrap();
    metadata.ino()
}

// This function takes the file name as an argument and returns the size in bytes
pub fn get_size(name: &str) -> u64 {
    let metadata = fs::metadata(name).unwrap();
    metadata.len()
}

pub fn get_permessions(name: &str, member: i32) -> [bool; 3] {
    let permessions: [bool; 3];
    let meta = fs::metadata(name).unwrap();
    let mode = meta.mode();

    match member {
        1 => {
            // Get User Permessions
            let user_read = mode & 0o400 != 0;
            let user_write = mode & 0o200 != 0;
            let user_exec = mode & 0o100 != 0;
            permessions = [user_read, user_write, user_exec];
        }
        2 => {
            // Get Group Permessions
            let group_read = mode & 0o040 != 0;
            let group_write = mode & 0o020 != 0;
            let group_exec = mode & 0o010 != 0;
            permessions = [group_read, group_write, group_exec];
        }
        3 => {
            //Get Others Permessions
            let others_read = mode & 0o004 != 0;
            let others_write = mode & 0o002 != 0;
            let others_exec = mode & 0o001 != 0;
            permessions = [others_read, others_write, others_exec];
        }
        _ => {
            unreachable!("Error no user indexed")
        }
    }
    permessions
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nFile Name : {}\nFull Path : {}\nOwned By User : {}\nOwned By Group : {}\nInode Number : {}\nLast Accessed At : {}\nLast Modified At : {}\nFile Size : { }\n",
            self.name,
            self.path,
            self.owner,
            self.group,
            self.inode,
            self.accesstime,
            self.modificationtime,
            self.size,
        );
        assert!(print_stdout(self.permessions.with_title()).is_ok());

        Ok(())
    }
}
