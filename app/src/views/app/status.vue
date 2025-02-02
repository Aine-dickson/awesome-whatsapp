<template>
    <div class="h-full grid statuss-main border-l border-slate-50 dark:border-slate-700">
        <div class="status-list border-r w-[20rem] border-slate-50 dark:border-gray-700 h-full grid gap-2">
            <div class="bg-slate-100 dark:bg-slate-800 header w-full h-full">
                <div class="flex w-full justify-between p-2">
                    <span class="font-bold text-2xl">Status</span>
                    <div>
                        <span>New</span>
                        <span>Menu</span>
                    </div>
                </div>
                <div @click="selectStatus(status.id)" v-for="(status, index) in statuses" :key="status.id" class="flex space-x-2 p-1 bg-slate-300 dark:bg-slate-700 cursor-pointer">
                    <div class="w-12 h-12 rounded-full overflow-hidden bg-amber-400">
                        <img src="" alt="" class="w-full h-full object-cover">
                    </div>
                    <div class="flex flex-col w-full">
                        <span>{{ status.username }}</span>
                        <span class="truncate">{{ status.messages[status.messages.length-1].message }}</span>
                    </div>
                </div>
            </div>
            <ul class="flex flex-col bg-slate-100 dark:bg-slate-800 h-full list w-full">
                <span class="p-4">RECENT</span>
                <li @click="selectStatus(status.id)" v-for="(status, index) in statuses" :key="status.id" class="flex space-x-2 p-1 bg-slate-300 dark:bg-slate-700 cursor-pointer">
                    <div class="w-12 h-12 rounded-full overflow-hidden bg-amber-400">
                        <img src="" alt="" class="w-full h-full object-cover">
                    </div>
                    <div class="flex flex-col w-full">
                        <span>{{ status.username }}</span>
                        <span class="truncate">{{ status.messages[status.messages.length-1].message }}</span>
                    </div>
                </li>
            </ul>
        </div>
        <div class="status_preview h-full" :class="{'flex flex-col justify-center items-center': !selectedStatus}">
            <div v-if="!selectedStatus" class="text-center flex flex-col">
                <span>No selected status!</span>
                <span>Select a status to preview</span>
            </div>
            <div v-else class="h-full">
                <header class="bg-slate-50 dark:bg-gray-800">
                    <div class="flex justify-between p-2">
                        <div class="flex flex-col">
                            <span class="font-bold text-lg">{{ selectedStatus.username }}</span>
                            <span class="text-sm text-slate-400">
                                last seen: 12:00
                            </span>
                        </div>
                        <div>
                            <span>Menu</span>
                        </div>
                    </div>
                </header>
                <main>
                    <ul class="p-4">
                        <li v-for="(message, index) in selectedStatus.messages" :key="message.id">
                            <div class="relative">
                                <span>{{ selectedStatus.username }}</span>
                                <div class="bg-slate-100 dark:bg-slate-950 rounded-lg p-2">
                                    {{ message.message }}
                                </div>
                                <div class="absolute -bottom-1.5 left-1.5 z-10">
                                    <img src="" alt="">
                                </div>
                            </div>
                            <div class="flex justify-end text-sm font-extralight">
                                <span>{{ message.time }}</span>
                            </div>
                        </li>
                    </ul>
                </main>
            </div>
        </div>
        <div class="status-info w-[20rem] border-l border-slate-50 dark:border-gray-700">
         
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue';

let selectedStatus: Ref<Status | null> = ref(null);

let statuses = ref([
    {
        id: 'wenjp34oem20',
        username: 'John Doe',
        messages: [
            {
                id: 'wemjwemwem',
                message: 'Hello',
                time: '12:00'
            },
            {
                id: 'wewlelormjwemwem',
                message: `How is the going over there, thus side it's been a tough week for me`,
                time: '12:01'
            }
        ]
    }
]);

type Status = {
    id: string;
    username: string;
    messages: StatusMessage[];
}

type StatusMessage = {
    id: string;
    message: string;
    time: string;
}

let selectStatus = (statusId: string) => {
    let target = statuses.value.find(status => status.id === statusId);
    if (target) {
        selectedStatus.value = target;
    }
}

</script>

<style scoped>
.statuss-main{
    grid-template-columns: min-content 1fr min-content;
    grid-template-rows: 100%;
    grid-template-areas: "status-list status_preview status-info";
}
.status-list{
    grid-area: status-list;
    grid-template-columns: 100%;
    grid-template-rows: min-content 1fr;
    grid-template-areas: "header" "list";
}
.status_preview{
    grid-area: status_preview;
}
.status-info{
    grid-area: status-info;
}
.header{
    grid-area: header;
}
.list{
    grid-area: list;
}
</style>