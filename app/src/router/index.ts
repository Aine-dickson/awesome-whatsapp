import { useAccountStore } from '@/stores/account'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'index',
      component: () => import('@/views/app/app.vue'),
      children: [
        {
          path: '/',
          name: 'chats',
          component: () => import('@/views/app/chats.vue'),
          meta: {
            auth_guarded: true
          }
        },
        {
          path: '/status',
          name: 'status',
          component: () => import('@/views/app/status.vue'),
          meta: {
            auth_guarded: true
          }
        },
        {
          path: '/calls',
          name: 'calls',
          component: () => import('@/views/app/calls.vue'),
          meta: {
            auth_guarded: true
          }
        }
      ]
    },
    {
      path: '/auth',
      name: 'auth',
      component: () => import('@/views/auth/index.vue'),
      children: [
        {
          path: '/login',
          name: 'login',
          component: () => import('@/views/auth/login.vue'),
          meta: {
            auth_guarded: false
          }
        },
        {
          path: '/signup',
          name: 'signup',
          component: () => import('@/views/auth/signup.vue'),
          meta: {
            auth_guarded: false
          }
        },
        {
          path: '/verify_contact',
          name: 'verify_contact',
          component: () => import('@/views/auth/verify_contact.vue'),
          meta: {
            auth_guarded: false
          }
        },
        {
          path: '/account_recovery',
          name: 'account_recovery',
          component: () => import('@/views/auth/recover_account.vue'),
        }
      ]
    }
  ],
})

router.beforeEach((to, from, next) => {
  let accountStore = useAccountStore()
  if (to.matched.some(record => record.meta.auth_guarded)) {
    if (accountStore.user == null) {
      console.log("Redirecting to login")
      next({
        name: 'login'
      })
    } else {
      next()
    }
  } else if(accountStore.user != null){
    next({name: from.name || 'home'})
  } else {
    next()
  }
})

export default router
