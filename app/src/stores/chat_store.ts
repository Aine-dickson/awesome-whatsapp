import { defineStore } from "pinia";

export const useChatStore = defineStore('chats', {
    state: () => {
        let chats: Chat[] = []
        let active_chat: Chat | null = null as Chat | null
        let active_filter: string = 'all'
        let creating_new_chat: boolean = false
        let contacts: Contact[] = [
            {name: 'John Doe', phone: '08012345678'},
            {name: 'Jane Doe', phone: '08014348979'},
        ]
        return {
            chats, active_chat, active_filter,
            creating_new_chat, contacts
        }
    },
    actions: {
        filter_chats(filter: 'all' | 'unread' | 'favorites' | 'groups'){
            this.active_filter = filter
        },

        select_contact(contact: string){
            let chat = this.chats.find(c => c.participants.includes(contact)) || this.create_chat(contact)
            this.active_chat = chat
            this.toggle_creation_state()
        },

        toggle_creation_state(){
            this.creating_new_chat = !this.creating_new_chat
        },

        preview_chat(chatId: string){
            this.active_chat = this.chats.find(c => c.id === chatId) || null
        },

        create_chat(chat: string){
            let new_chat: Chat = {
                id: Math.random().toString(36).substring(7),
                name: chat,
                messages: [],
                participants: [chat]
            }
            this.chats.push(new_chat)
            return new_chat
        },

        delete_chat(chat: Chat){
            this.chats = this.chats.filter(c => c.id !== chat.id)
        }
    }
})

interface Chat {
    id: string
    name: string
    messages: Message[]
    participants: string[]
}

interface Message {
    id: string
    content: string
    sender: string
    receiver: string
    sent_at: string
}

interface Contact {
    name: string
    phone: string
}