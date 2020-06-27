/// 遠隔操作プロジェクト用の共有ライブラリ
pub mod remote_control {
    extern crate bincode;
    extern crate serde;
    extern crate serde_derive;
    
    use serde_derive::*;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::fmt;
    
    pub enum ParseKind {
        Echo(String),
        RunCommand { command: String, is_waiting: bool },
        SendFile { filename: String, contents: String },
        End,
        Ls,
        Help,
    }
    
    /// データ送信の際の型
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub enum MyMessage {
        Echo(String),
        RunCommand { command: String, is_waiting: bool },
        SendFile { filename: String, contents: String },
    }
    
    /// データ受信の際の型の種類
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub enum MyResponseKind {
        Echo(String),
        RunCommand { stdout: String, stderr: String },
        SendFile,
    }
    
    /// データ受信の際の型
    pub type MyResponse = Result<MyResponseKind, MyError>;
    
    /// エラーメッセージとエラー発生場所の種類
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct MyError {
        pub msg: String,
        pub when: String,
    }
    
    impl MyError {
        pub fn new<T: Display>(t: T, when: String) -> Self {
            MyError {
                msg: t.to_string(),
                when,
            }
        }
    }
    
    impl Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "msg : {}\nwhen : {}", self.msg, self.when)
        }
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct MyConfig {
        pub ip: String,
        pub port: String,
    }
    
    impl MyConfig {
        pub fn from_configfile(path: &Path) -> Result<Self, MyError> {
            let mut configfile = File::open(path).map_err(|e| {
                MyError::new(
                    e,
                    "Configファイルを開く際にエラーが発生しました".to_string(),
                )
            })?;
            
            let mut buf = String::new();
            configfile.read_to_string(&mut buf).map_err(|e| {
                MyError::new(
                    e,
                    "Configファイルの読み込みの際にエラーが発生しました".to_string(),
                )
            })?;
            
            let mut input = buf.split_whitespace();
            let ip = input.next().ok_or_else(|| {
                MyError::new(
                    "ipアドレスが指定されていません".to_string(),
                    "Configファイルの解析の際にエラーが発生しました".to_string(),
                )
            })?.to_string();
            
            let port = input.next().ok_or_else(|| {
                MyError::new(
                    "Portが指定されていません".to_string(),
                    "Configファイルの解析の際にエラーが発生しました".to_string(),
                )
            })?.to_string();
            
            Ok(MyConfig { ip, port })
        }
    }
    
    pub fn log_error(me: MyError) {
        let logfile_path: &Path = Path::new("err.log");
        let mut logfile = if logfile_path.exists() {
            File::open(logfile_path).unwrap()
        } else {
            File::create(logfile_path).unwrap()
        };
        
        let log = format!("{:?}\n", me);
        eprintln!("{}", log);
        logfile.write_all((&log[..]).as_ref()).unwrap();
        logfile.flush().unwrap();
    }
}
