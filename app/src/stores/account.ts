import { defineStore } from 'pinia'
import { api } from '@/router/api'
import { ErrorSource, useErrorStore, type Error } from './error_handler'
import type Verify_contact from '@/views/auth/verify_contact.vue'

export const useAccountStore = defineStore('account', {
  state: ()=> {
    let user: null | LoggedInUser = null
    let theme = Theme.light as Theme
    let drawer = false
    let active_nav = 'home'
    return {
      user, theme, drawer, active_nav
    }
  },
  actions: {
    toggle_theme(){
      let new_theme = this.theme === Theme.light ? Theme.dark : Theme.light
      this.theme = new_theme
      let app = document.querySelector('#app')
      if (app){
        app.classList.remove(this.theme === Theme.light ? 'dark' : 'light')
        app.classList.add(this.theme === Theme.light ? 'light' : 'dark')
      }
    },

    toggleDrawer() {
      this.drawer = !this.drawer
    },

    activateNav(target: string) {
      this.active_nav = target
    },

    async login(user: User){
      try {
        let response = await api.post('/user/login', user)
        if (response.status === 200){
          this.user = response.data
          return true
        }
      } catch (error) {
        console.log(error)
        return false
      }
    },

    async signup(user: User){
      let error_handler = useErrorStore()
      try {
        let response = await api.post('/user/signup', user)
        if (response.status === 200){
          this.user = response.data
          return true
        } else {
          let new_error: Error = {
            message: response.data,
            status: response.status,
            source: ErrorSource.UserAction
          }
          error_handler.setError(new_error)
          return false
        }
      } catch (error) {
        
        return false
      }
    },

    async logout(){
      try {
        let response = await api.post('/user/logout')
        if (response.status === 200){
          this.user = null
          return true
        }
      } catch (error) {
        console.log(error)
        return false
      }
    },

    async verify_contact(code: string){
      try {
        let response = await api.get('/user/verify_contact')
        if (response.status === 200){
          this.user = response.data
          return true
        }
      } catch (error) {
        console.log(error)
        return false
      }
    },

    async resend_verification(){

    },

    async recover_account(email: string){
      try {
        let response = await api.post('/user/recover_account', {email})
        if (response.status === 200){
          return true
        }
      } catch (error) {
        console.log(error)
        return false
      }
    }
  },
  persist: true
})


export interface User{
  tel_contact: string
  passcode: string
}

interface LoggedInUser extends User{
  username: string,
  createdAt: string,
  updatedAt: string,
  verified: boolean
}

enum Theme{
  light = 'light',
  dark = 'dark'
}
