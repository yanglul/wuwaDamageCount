import { createApp } from "vue";
// import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus' //全局引入
import 'element-plus/dist/index.css'
const app = createApp(App)
 
app.use(ElementPlus)
app.mount('#app')