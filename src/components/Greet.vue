<template>
  <p align="center">计算共鸣技能伤害</p>
  <el-form ref="form" style="text-align: center">
    
    <el-form-item label="攻击" :label-width="formLabelWidth">
      <el-input v-model="atk"  ></el-input>
    </el-form-item>
    <el-form-item label="角色等级" :label-width="formLabelWidth" >
      <el-input v-model="userLevel"  ></el-input>
    </el-form-item>
    <el-form-item label="暴击率" :label-width="formLabelWidth" >
      <el-input v-model="critRate"  ></el-input>
    </el-form-item>
    <el-form-item label="暴击伤害" :label-width="formLabelWidth" >
      <el-input v-model="critDmg"  ></el-input>
    </el-form-item>
    <!-- <el-form-item label="共鸣效率" :label-width="formLabelWidth" >
      <el-input v-model="resonanceEfficiency"  ></el-input>
    </el-form-item> -->
    <el-form-item label="共鸣技能伤害加成" :label-width="formLabelWidth" >
      <el-input v-model="resonanceDamageBonus"  ></el-input>
    </el-form-item>
    <!-- <el-form-item label="普攻伤害加成" :label-width="formLabelWidth" >
      <el-input v-model="basic_attack_damage_bonus"  ></el-input>
    </el-form-item>
    <el-form-item label="重击伤害加成" :label-width="formLabelWidth" >
      <el-input v-model="bash_damage_bonus"  ></el-input>
    </el-form-item> -->
    <!-- <el-form-item label="共鸣解放伤害加成" :label-width="formLabelWidth" >
      <el-input v-model="bash_damage_bonus"  ></el-input>
    </el-form-item> -->
    <el-form-item label="属性伤害加成" :label-width="formLabelWidth"  >
      <el-input v-model="attributeDamageBonus"  ></el-input>
    </el-form-item>


    <el-form-item label="共鸣技能技能倍率" :label-width="formLabelWidth" >
      <el-input v-model="skillMultiplier"  ></el-input>
    </el-form-item>

    <el-form-item label="倍率提升"  :label-width="formLabelWidth">
      <el-input v-model="multiplierIncrease"  ></el-input>
    </el-form-item>
    <el-form-item label="伤害加深"  :label-width="formLabelWidth" >
      <el-input v-model="damageAmplification"  ></el-input>
    </el-form-item>

    <el-form-item label="怪物等级"  :label-width="formLabelWidth" >
      <el-input v-model="monsterLevel"  ></el-input>
    </el-form-item>
    <el-form-item label="属性伤害减免" :label-width="formLabelWidth" >
      <el-input v-model="elementalDamageReduction"  ></el-input>
    </el-form-item>
    <el-form-item label="属性抗性" :label-width="formLabelWidth"  >
      <el-input v-model="attributeResistance"  ></el-input>
    </el-form-item>
    <el-button align="center"  type="primary" @click.prevent="count()">计算</el-button>
  </el-form>


  <p>{{ greetMsg }}</p>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
 
const formLabelWidth = '180px';

const atk =  ref(1);
const userLevel = ref(1);
const critRate = ref(0.05);
const critDmg = ref(1.5);
// const resonanceEfficiency = 1;
const resonanceDamageBonus = ref(1);
// const basic_attack_damage_bonus = 1;
// const bash_damage_bonus = 1;
const attributeDamageBonus = ref(1);
const skillMultiplier = ref(1);
const multiplierIncrease = ref(0);
const monsterLevel = ref(1);
const damageAmplification = ref(0);
const elementalDamageReduction = ref(0);
const attributeResistance = ref(0.1);

const greetMsg = ref("所有百分比数值需转换成小数");
 
class RES {
  "success": string;
  "data": string;
}

 

async function count() {
  let startServer: RES = await invoke("conutDamage", { 
    atk:  Number(atk.value) ,
    userLevel: Number(userLevel.value),
    critRate: Number(critRate.value),
    critDmg:  Number(critDmg.value),
    resonanceDamageBonus: Number(resonanceDamageBonus.value),
    attributeDamageBonus: Number(attributeDamageBonus.value),
    skillMultiplier: Number(skillMultiplier.value),
    multiplierIncrease:Number(multiplierIncrease.value),
    monsterLevel:Number(monsterLevel.value),
    damageAmplification:Number(damageAmplification.value),
    elementalDamageReduction:Number(elementalDamageReduction.value),
    attributeResistance:Number(attributeResistance.value),
  });
  greetMsg.value = "计算返回:" + startServer.data;

}
</script>
