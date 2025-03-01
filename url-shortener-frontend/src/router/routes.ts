import type { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path:'/',
    component: () => import('pages/UrlPage.vue'),
  },
];

export default routes;
