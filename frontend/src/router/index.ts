import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login',    name: 'login',    component: () => import('@/views/LoginView.vue'),     meta: { public: true } },
    { path: '/signup',   name: 'signup',   component: () => import('@/views/SignupView.vue'),    meta: { public: true } },
    { path: '/',         name: 'dashboard',component: () => import('@/views/DashboardView.vue') },
    { path: '/certs',    name: 'certs',    component: () => import('@/views/MyCertView.vue') },
    { path: '/goals',    name: 'goals',    component: () => import('@/views/GoalView.vue') },
    { path: '/users',    name: 'users',    component: () => import('@/views/UsersView.vue') },
    { path: '/:pathMatch(.*)*', redirect: '/' },
  ],
})

router.beforeEach(async (to) => {
  const auth = useAuthStore()

  if (auth.user === null && !auth.loading) {
    await auth.fetchMe()
  }

  if (!to.meta.public && !auth.isLoggedIn()) {
    return { name: 'login' }
  }

  if (to.meta.public && auth.isLoggedIn()) {
    return { name: 'dashboard' }
  }
})

export default router
