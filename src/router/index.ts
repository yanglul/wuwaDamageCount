import {createRouter,createWebHistory,RouteRecordRaw} from 'vue-router'

const routes: Array<RouteRecordRaw> = [
    {
        path:'/home',
        name:'home',
        component:()=> import(/*  webpackChunkName:"home"   */'../views/HomeView.vue'),
    },
    
]
const router = createRouter({
    history: createWebHistory(),
    routes,
  });
   
// 3导出路由   然后去 main.ts 注册 router.ts
export default router