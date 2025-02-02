<template>
    <div class="h-full chats-main" :class="{'grid': !chatStore.creating_new_chat}">
        <!-- New chat creation section -->
        <div v-if="chatStore.creating_new_chat" class="chat-list h-full border-r w-[20rem] border-slate-50 dark:border-slate-700">
            <div class="header full">
                <div class="flex p-4 space-x-10">
                    <svg @click="chatStore.toggle_creation_state" class="w-6 h-6 cursor-pointer text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 12h14M5 12l4-4m-4 4 4 4"/>
                    </svg>
                    <span class="font-bold text-lg">New chat</span>
                </div>

                <!-- Search new chat -->
                <div>
                    <search_contacts/>
                </div>
            </div>
            <ul class="list h-full overflow-hidden overflow-y-auto">
                <li class="p-2">
                    <div>
                        <img src="" alt="">
                    </div>
                    <span>New group</span>
                </li>
                <span class="block uppercase font-light text-lg p-2 py-4">Contacts in your tree</span>

                <!-- List of contacts -->
                <li @click="chatStore.select_contact(contact.phone)" v-for="(contact, index) in chatStore.contacts" :key="contact.phone" class="flex space-x-2 p-2 cursor-pointer">
                    <contact :contact="contact"/>
                </li>
            </ul>
        </div>

        <div v-else class="chat-list border-r w-[20rem] border-slate-50 dark:border-slate-700 grid">
            <div class="header h-full">
                <div class="flex w-full justify-between p-2">
                    <span class="font-bold text-2xl">Chats</span>
                    <div class="flex items-center space-x-2">
                        <div @click="chatStore.toggle_creation_state" class="group relative inline-block">
                            <svg class="w-6 h-6 text-slate-800 dark:text-white cursor-pointer" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>
                            </svg>
                            <span class="absolute z-10 hidden group-hover:block text-sm font-light whitespace-nowrap bg-slate-900 border border-slate-400 rounded-sm p-1">New chat</span>
                        </div>
                        <div class="group relative inline-block">
                            <svg class="w-6 h-6 text-slate-800 dark:text-white cursor-pointer" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-width="4" d="M12 6h.01M12 12h.01M12 18h.01"/>
                            </svg>
                            <span class="absolute z-10 hidden group-hover:block text-sm font-light whitespace-nowrap bg-slate-900 border border-slate-400 rounded-sm p-1">Menu</span>
                        </div>
                    </div>
                </div>

                <!-- Search input -->
                <div>
                    <search_chats/>
                </div>

                <!-- Chat Filters -->
                <div class="flex filters space-x-2 p-2 ">
                    <span @click="chatStore.filter_chats('all')" class="rounded-md cursor-pointer p-1 px-2 text-sm" :class="{'bg-slate-300 dark:bg-slate-700 ': chatStore.active_filter != 'all', 'bg-blue-500 text-white': chatStore.active_filter == 'all'}">All</span>
                    <span @click="chatStore.filter_chats('unread')" class="rounded-md cursor-pointer p-1 px-2 text-sm" :class="{'bg-slate-300 dark:bg-slate-700': chatStore.active_filter != 'unread', 'bg-blue-500 text-white': chatStore.active_filter == 'unread'}">Unread</span>
                    <span @click="chatStore.filter_chats('favorites')" class="rounded-md cursor-pointer p-1 px-2 text-sm" :class="{'bg-slate-300 dark:bg-slate-700': chatStore.active_filter != 'favorites', 'bg-blue-500 text-white': chatStore.active_filter == 'favorites'}">Favorites</span>
                    <span @click="chatStore.filter_chats('groups')" class="rounded-md cursor-pointer p-1 px-2 text-sm" :class="{'bg-slate-300 dark:bg-slate-700': chatStore.active_filter != 'groups', 'bg-blue-500 text-white': chatStore.active_filter == 'groups'}">Groups</span>
                </div>
            </div>

            <!-- Chat list -->
            <ul class="flex flex-col list h-full overflow-hidden overflow-y-auto">
                <div v-if="chatStore.chats.length < 1" class="flex flex-col items-center justify-center h-full">
                    <span>Start by creating a <span @click="chatStore.toggle_creation_state" class="underline underline-offset-1 text-blue-500 cursor-pointer">new chat</span></span>
                </div>

                <!-- List of chats -->
                <li v-else @click="chatStore.preview_chat(chat.id)" v-for="(chat, index) in chatStore.chats" :key="chat.id" class="flex space-x-2 p-1 bg-slate-300 dark:bg-slate-700 cursor-pointer">
                   <Chat :chat="chat"/>
                </li>
            </ul>
        </div>

        <div class="chat_preview h-full" :class="{'flex flex-col justify-center items-center': !chatStore.active_chat}">
            <div v-if="!chatStore.active_chat" class="text-center flex flex-col">
                <span>No selected chat!</span>
                <span>Select a chat to preview</span>
            </div>
            <div v-else class="h-full grid messages">

                <header class="bg-slate-50 dark:bg-slate-800 messages-header">
                    <div class="flex justify-between p-2">
                        <div class="flex flex-col">
                            <span class="font-bold text-lg">{{ chatStore.active_chat.name }}</span>
                            <span class="text-sm text-slate-400">
                                last seen: 12:00
                            </span>
                        </div>
                        <div>
                            <span>Menu</span>
                        </div>
                    </div>
                </header>

                <main class="messages-list h-full overflow-hidden overflow-y-auto">
                    <div v-if="chatStore.active_chat.messages.length < 1" class="h-full flex flex-col font-light text-gray-400 items-center justify-center">
                        <span>One message leads to another</span>
                        <span>Start by typing that message below!</span>
                    </div>

                    <!-- Messages for open chat -->
                    <ul v-else class="p-4">
                        <li v-for="(message, index) in chatStore.active_chat.messages" :key="message.id">
                            <Message :message="message"/>
                        </li>
                    </ul>
                </main>

                <footer class="messages-footer">
                    <MessageInput/>
                </footer>
            </div>
        </div>

        <div class="chat-info w-[20rem] border-l border-slate-50 dark:border-slate-700">
         
        </div>
    </div>
</template>

<script setup lang="ts">
import { useChatStore } from '@/stores/chat_store';
import Message from '@/components/chats/message.vue';
import MessageInput from '@/components/chats/input.vue';
import search_chats from '@/components/chats/search_chats.vue';
import Chat from '@/components/chats/chat.vue';
import search_contacts from '@/components/chats/search_contacts.vue';
import contact from '@/components/chats/contact.vue';

let chatStore = useChatStore();

</script>

<style scoped>
.chats-main{
    grid-template-columns: min-content 1fr min-content;
    grid-template-rows: 100%;
    grid-template-areas: "chat-list chat_preview chat-info";
}
.chat-list{
    grid-area: chat-list;
    grid-template-columns: 100%;
    grid-template-rows: min-content 1fr;
    grid-template-areas: "header" "list";
}
.messages{
    grid-template-columns: 100%;
    grid-template-rows: min-content 1fr auto;
    grid-template-areas: "msg-header" "msg-list" "msg-footer";
}
.messages-header{
    grid-area: msg-header;
}
.messages-list{
    grid-area: msg-list;
}
.messages-footer{
    grid-area: msg-footer;
}
.chat_preview{
    grid-area: chat_preview;
}
.chat-info{
    grid-area: chat-info;
}
.header{
    grid-area: header;
}
.list{
    grid-area: list;
}
</style>