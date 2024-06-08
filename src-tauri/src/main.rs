// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const SYS_FILE_NAME:&str = "app.config";
 

use serde::{Serialize, Deserialize};

#[derive(Debug,Default,Serialize, Deserialize)]
pub struct Player {
    basic:u32,
    level:u32,
    crit_rate:f32,
    crit_dmg:f32,
    resonance_damage_bonus:f32,
    attribute_damage_bonus:f32,
}

 
impl Player{
    fn conutCrit(&self,skill_multiplier:f32,multiplier_increase:f32,damage_amplification:f32,monster_level:u32,attribute_resistance:f32,elemental_damage_reduction:f32) -> (u32,u32,u32) {
        println!("1 {:?}",self.basic as f32);
        println!("2 {:?}",skill_multiplier);
        println!("3 {:?}",(1.0f32 + multiplier_increase));
        println!("4 {:?}",(1.0f32 +damage_amplification));
        println!("5 {:?}",(1.0f32 +self.attribute_damage_bonus+self.resonance_damage_bonus));
        println!("6 {:?}",((100 +self.level)as f32 / (199+monster_level+self.level) as f32));
        println!("7 {:?}",(1.0f32 -attribute_resistance));
        println!("8 {:?}",(1.0f32 -elemental_damage_reduction));

        let b = self.basic as f32 * 
        skill_multiplier * 
        (1.0f32 + multiplier_increase) * 
        (1.0f32 +damage_amplification) * 
        (1.0f32 +self.attribute_damage_bonus+self.resonance_damage_bonus)*
        ((100 +self.level)as f32 / (199+monster_level+self.level) as f32) *
        (1.0f32 -attribute_resistance) *
        (1.0f32 -elemental_damage_reduction);
        
        let crit =   b*self.crit_dmg  ;
        let expec  = b * (1.0f32+self.crit_rate*(self.crit_dmg-1.0f32));

        (b.ceil() as u32,crit.ceil() as u32,expec.ceil() as u32)


    }
}




 
#[derive(Serialize, Deserialize)]
pub struct Res{
    success: String,
    data:Option<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
 

#[tauri::command]
fn loadServerConfig(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
 

#[tauri::command]
fn conutDamage(
    ATK: u32,
    user_level:u32,
    crit_rate:f32,
    crit_dmg:f32,
    resonance_damage_bonus:f32,
    attribute_damage_bonus:f32,
    skill_multiplier:f32,
    multiplier_increase:f32,
    monster_level:u32,
    damage_amplification:f32,
    elemental_damage_reduction:f32,
    attribute_resistance:f32,

) -> Res {
    let p = Player{
        basic:ATK,
        level:user_level,
        crit_rate:crit_rate,
        crit_dmg:crit_dmg,
        resonance_damage_bonus:resonance_damage_bonus,
        attribute_damage_bonus:attribute_damage_bonus,
    };
    println!("{:?},{},{},{},{},{},{}",p,skill_multiplier, multiplier_increase, damage_amplification, monster_level, attribute_resistance, elemental_damage_reduction);
    let (a,b,c) = p.conutCrit(skill_multiplier, multiplier_increase, damage_amplification, monster_level, attribute_resistance, elemental_damage_reduction);
    Res{
        success: 0.to_string(),
        data:Some(format!("共鸣技能未暴击时:{},暴击时:{},伤害期望:{}",a,b,c))
        }
}




fn main() {
   
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![conutDamage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
