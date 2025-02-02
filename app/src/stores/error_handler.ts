import { defineStore } from "pinia";

export const useErrorStore = defineStore('error', {
    state: ()=> {
        let error = null as Error | null

        return{
            error
        }
    },
    actions: {
        setError(error: Error){
            this.error = error as Error
            setTimeout(()=>{
                this.clearError()
            }, 5000)
        },
        clearError(){
            this.error = null
        }
    }
})

export interface Error {
    message: string
    status: number
    source: ErrorSource
}

export enum ErrorSource {
    Network,
    UserAction,
}