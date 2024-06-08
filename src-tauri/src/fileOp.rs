use std::{fs, ptr::null};
use serde_json::*;
use crate::Res;

const SYS_FILE_NAME:&str = "app.config";

pub fn loadFile(path: &str)->Result<Res>{
    let file_content = fs::read_to_string(path);
    match  file_content {
        Ok(content) =>{

            let data:Value = serde_json::from_str(&content)?;
            let data:String = data["data"].to_string();
            let r = Res{
                success:"0".to_string(),
                data:Some(data),
            };
            return Ok(r);
        },
        Err(e) =>{
            let r = Res{
                success:"10001".to_string(),
                data:Some(e.to_string()),
            };
            return Ok(r);
        }
    }

    
}





pub fn writeSysConfig(config_path:&str)->Res{
    let r = Res{
        success:"0".to_string(),
        data:Some(config_path.to_string()),
    };
    let v = json!(r);
    let res = fs::write(SYS_FILE_NAME, v.to_string());
    match res {
        Err(e) =>{
            let re = Res{
                success:"10002".to_string(),
                data:Some("写入文件失败".to_string()),
            };
            return re;
        },
        _ =>{return r},
    }

}